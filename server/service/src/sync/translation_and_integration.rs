use crate::{
    sync::{integrate_document::sync_upsert_document, translations::PullDeleteRecordTable},
    usize_to_u64,
};

use super::{
    sync_buffer::SyncBuffer,
    sync_status::logger::{SyncLogger, SyncLoggerError, SyncStepProgress},
    translations::{
        IntegrationRecords, PullDeleteRecord, PullUpsertRecord, SyncTranslation, SyncTranslators,
    },
};
use log::warn;
use repository::*;
use std::collections::HashMap;

static PROGRESS_STEP_LEN: usize = 100;

pub(crate) struct TranslationAndIntegration<'a> {
    connection: &'a StorageConnection,
    sync_buffer: &'a SyncBuffer<'a>,
}

#[derive(Default, Debug)]
pub(crate) struct TranslationAndIntegrationResult {
    pub(crate) integrated_count: u32,
    pub(crate) errors_count: u32,
}
type TableName = String;
#[derive(Default, Debug)]
pub struct TranslationAndIntegrationResults(HashMap<TableName, TranslationAndIntegrationResult>);

impl<'a> TranslationAndIntegration<'a> {
    pub(crate) fn new(
        connection: &'a StorageConnection,
        sync_buffer: &'a SyncBuffer,
    ) -> TranslationAndIntegration<'a> {
        TranslationAndIntegration {
            connection,
            sync_buffer,
        }
    }

    // Go through each translator, adding translations to result, if no translators matched return None
    fn translate_sync_record(
        &self,
        sync_record: &SyncBufferRow,
        translators: &SyncTranslators,
    ) -> Result<Option<IntegrationRecords>, anyhow::Error> {
        let mut translation_results = IntegrationRecords::new();

        for translator in translators.iter() {
            let translation_result = match sync_record.action {
                SyncBufferAction::Upsert => {
                    translator.try_translate_pull_upsert(self.connection, &sync_record)?
                }
                SyncBufferAction::Delete => {
                    translator.try_translate_pull_delete(self.connection, &sync_record)?
                }
                SyncBufferAction::Merge => return Err(anyhow::anyhow!("Merge not implemented")),
            };

            if let Some(translation_result) = translation_result {
                translation_results = translation_results.join(translation_result);
            }
        }

        if translation_results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(translation_results))
        }
    }

    pub(crate) fn translate_and_integrate_sync_records(
        &self,
        sync_records: Vec<SyncBufferRow>,
        translators: &Vec<Box<dyn SyncTranslation>>,
        mut logger: Option<&mut SyncLogger>,
    ) -> Result<TranslationAndIntegrationResults, RepositoryError> {
        let step_progress = SyncStepProgress::Integrate;
        let mut result = TranslationAndIntegrationResults::new();

        // Record initial progress (will be set as total progress)
        let total_to_integrate = sync_records.len();

        // Helper to make below logic less verbose
        let mut record_progress = |progress: usize| -> Result<(), RepositoryError> {
            match logger.as_mut() {
                None => Ok(()),
                Some(logger) => logger
                    .progress(step_progress.clone(), usize_to_u64(progress))
                    .map_err(SyncLoggerError::to_repository_error),
            }
        };

        for (number_of_records_integrated, sync_record) in sync_records.into_iter().enumerate() {
            let translation_result = match self.translate_sync_record(&sync_record, &translators) {
                Ok(translation_result) => translation_result,
                // Record error in sync buffer and in result, continue to next sync_record
                Err(translation_error) => {
                    self.sync_buffer
                        .record_integration_error(&sync_record, &translation_error)?;
                    result.insert_error(&sync_record.table_name);
                    warn!(
                        "{:?} {:?} {:?}",
                        translation_error, sync_record.record_id, sync_record.table_name
                    );
                    // Next sync_record
                    continue;
                }
            };

            let integration_records = match translation_result {
                Some(integration_records) => integration_records,
                // Record translator not found error in sync buffer and in result, continue to next sync_record
                None => {
                    let error = anyhow::anyhow!("Translator for record not found");
                    self.sync_buffer
                        .record_integration_error(&sync_record, &error)?;
                    result.insert_error(&sync_record.table_name);
                    warn!(
                        "{:?} {:?} {:?}",
                        error, sync_record.record_id, sync_record.table_name
                    );
                    // Next sync_record
                    continue;
                }
            };

            // Integrate
            let integration_result = integration_records.integrate(self.connection);
            match integration_result {
                Ok(_) => {
                    self.sync_buffer
                        .record_successful_integration(&sync_record)?;
                    result.insert_success(&sync_record.table_name)
                }
                // Record database_error in sync buffer and in result
                Err(database_error) => {
                    let error = anyhow::anyhow!("{:?}", database_error);
                    self.sync_buffer
                        .record_integration_error(&sync_record, &error)?;
                    result.insert_error(&sync_record.table_name);
                    warn!(
                        "{:?} {:?} {:?}",
                        error, sync_record.record_id, sync_record.table_name
                    );
                }
            }

            if number_of_records_integrated % PROGRESS_STEP_LEN == 0 {
                record_progress(total_to_integrate - number_of_records_integrated)?;
            }
        }

        // Record final progress
        record_progress(0)?;

        Ok(result)
    }
}

impl IntegrationRecords {
    pub(crate) fn integrate(&self, connection: &StorageConnection) -> Result<(), RepositoryError> {
        // Only start nested transaction if transaction is already ongoing. See integrate_and_translate_sync_buffer
        let start_nested_transaction = { connection.transaction_level.get() > 0 };

        for delete in self.deletes.iter() {
            // Integrate every record in a sub transaction. This is mainly for Postgres where the
            // whole transaction fails when there is a DB error (not a problem in sqlite).
            if start_nested_transaction {
                connection
                    .transaction_sync_etc(|sub_tx| delete.delete(sub_tx), false)
                    .map_err(|e| e.to_inner_error())?;
            } else {
                delete.delete(&connection)?;
            }
        }

        for upsert in self.upserts.iter() {
            // Integrate every record in a sub transaction. This is mainly for Postgres where the
            // whole transaction fails when there is a DB error (not a problem in sqlite).
            if start_nested_transaction {
                connection
                    .transaction_sync_etc(|sub_tx| upsert.upsert(sub_tx), false)
                    .map_err(|e| e.to_inner_error())?;
            } else {
                upsert.upsert(&connection)?;
            }
        }

        Ok(())
    }
}

impl PullUpsertRecord {
    pub(crate) fn upsert(&self, con: &StorageConnection) -> Result<(), RepositoryError> {
        use PullUpsertRecord::*;
        match self {
            UserPermission(record) => UserPermissionRowRepository::new(con).upsert_one(record),
            Name(record) => NameRowRepository::new(con).sync_upsert_one(record),
            NameTag(record) => NameTagRowRepository::new(con).upsert_one(record),
            NameTagJoin(record) => NameTagJoinRepository::new(con).upsert_one(record),
            Unit(record) => UnitRowRepository::new(con).upsert_one(record),
            Item(record) => ItemRowRepository::new(con).upsert_one(record),
            Store(record) => StoreRowRepository::new(con).upsert_one(record),
            MasterList(record) => MasterListRowRepository::new(con).upsert_one(record),
            MasterListLine(record) => MasterListLineRowRepository::new(con).upsert_one(record),
            MasterListNameJoin(record) => MasterListNameJoinRepository::new(con).upsert_one(record),
            PeriodSchedule(record) => PeriodScheduleRowRepository::new(con).upsert_one(record),
            Period(record) => PeriodRowRepository::new(con).upsert_one(record),
            Context(record) => ContextRowRepository::new(con).upsert_one(record),
            Program(record) => ProgramRowRepository::new(con).upsert_one(record),
            ProgramRequisitionSettings(record) => {
                ProgramRequisitionSettingsRowRepository::new(con).upsert_one(record)
            }
            ProgramRequisitionOrderType(record) => {
                ProgramRequisitionOrderTypeRowRepository::new(con).upsert_one(record)
            }
            Report(record) => ReportRowRepository::new(con).upsert_one(record),
            Location(record) => LocationRowRepository::new(con).upsert_one(record),
            LocationMovement(record) => LocationMovementRowRepository::new(con).upsert_one(record),
            StockLine(record) => StockLineRowRepository::new(con).upsert_one(record),
            NameStoreJoin(record) => NameStoreJoinRepository::new(con).sync_upsert_one(record),
            Invoice(record) => InvoiceRowRepository::new(con).upsert_one(record),
            InvoiceLine(record) => InvoiceLineRowRepository::new(con).upsert_one(record),
            Stocktake(record) => StocktakeRowRepository::new(con).upsert_one(record),
            StocktakeLine(record) => StocktakeLineRowRepository::new(con).upsert_one(record),
            Requisition(record) => RequisitionRowRepository::new(con).sync_upsert_one(record),
            RequisitionLine(record) => {
                RequisitionLineRowRepository::new(con).sync_upsert_one(record)
            }
            ActivityLog(record) => ActivityLogRowRepository::new(con).insert_one(record),
            InventoryAdjustmentReason(record) => {
                InventoryAdjustmentReasonRowRepository::new(con).upsert_one(record)
            }
            StorePreference(record) => StorePreferenceRowRepository::new(con).upsert_one(record),
            Barcode(record) => BarcodeRowRepository::new(con).sync_upsert_one(record),
            Sensor(record) => SensorRowRepository::new(con).upsert_one(record),
            TemperatureLog(record) => TemperatureLogRowRepository::new(con).upsert_one(record),
            TemperatureBreach(record) => {
                TemperatureBreachRowRepository::new(con).upsert_one(record)
            }
            Clinician(record) => ClinicianRowRepository::new(con).sync_upsert_one(record),
            ClinicianStoreJoin(record) => {
                ClinicianStoreJoinRowRepository::new(con).sync_upsert_one(record)
            }
            FormSchema(record) => FormSchemaRowRepository::new(con).upsert_one(record),
            DocumentRegistry(record) => DocumentRegistryRowRepository::new(con).upsert_one(record),
            Document(record) => sync_upsert_document(con, record),
        }
    }
}

impl PullDeleteRecord {
    pub(crate) fn delete(&self, con: &StorageConnection) -> Result<(), RepositoryError> {
        use PullDeleteRecordTable::*;
        let id = &self.id;
        match self.table {
            UserPermission => UserPermissionRowRepository::new(con).delete(id),
            Name => NameRowRepository::new(con).delete(id),
            NameTagJoin => NameTagJoinRepository::new(con).delete(id),
            Unit => UnitRowRepository::new(con).delete(id),
            Item => ItemRowRepository::new(con).delete(id),
            Store => StoreRowRepository::new(con).delete(id),
            ProgramRequisitionOrderType => {
                ProgramRequisitionOrderTypeRowRepository::new(con).delete(id)
            }
            ProgramRequisitionSettings => {
                ProgramRequisitionSettingsRowRepository::new(con).delete(id)
            }
            MasterListNameJoin => MasterListNameJoinRepository::new(con).delete(id),
            Report => ReportRowRepository::new(con).delete(id),
            NameStoreJoin => NameStoreJoinRepository::new(con).delete(id),
            Invoice => InvoiceRowRepository::new(con).delete(id),
            InvoiceLine => InvoiceLineRowRepository::new(con).delete(id),
            Requisition => RequisitionRowRepository::new(con).delete(id),
            RequisitionLine => RequisitionLineRowRepository::new(con).delete(id),
            InventoryAdjustmentReason => {
                InventoryAdjustmentReasonRowRepository::new(con).delete(id)
            }
            #[cfg(all(test, feature = "integration_test"))]
            Location => LocationRowRepository::new(con).delete(id),
            #[cfg(all(test, feature = "integration_test"))]
            StockLine => StockLineRowRepository::new(con).delete(id),
            #[cfg(all(test, feature = "integration_test"))]
            Stocktake => StocktakeRowRepository::new(con).delete(id),
            #[cfg(all(test, feature = "integration_test"))]
            StocktakeLine => StocktakeLineRowRepository::new(con).delete(id),
            #[cfg(all(test, feature = "integration_test"))]
            ActivityLog => Ok(()),
        }
    }
}

impl TranslationAndIntegrationResults {
    fn new() -> TranslationAndIntegrationResults {
        Default::default()
    }

    fn insert_error(&mut self, table_name: &str) {
        let entry = self
            .0
            .entry(table_name.to_owned())
            .or_insert(Default::default());
        entry.errors_count += 1;
    }

    fn insert_success(&mut self, table_name: &str) {
        let entry = self
            .0
            .entry(table_name.to_owned())
            .or_insert(Default::default());
        entry.integrated_count += 1;
    }
}

#[cfg(test)]
mod test {
    use repository::{
        mock::{MockData, MockDataInserts},
        test_db, ItemRow, ItemRowRepository, RepositoryError, UnitRow, UnitRowRepository,
    };
    use util::{assert_matches, bench_point, bench_results, inline_init, uuid::uuid};

    use crate::sync::translations::{IntegrationRecords, PullUpsertRecord};

    #[actix_rt::test]
    async fn test_fall_through_inner_transaction() {
        let (_, connection, _, _) = test_db::setup_all(
            "test_fall_through_inner_transaction",
            MockDataInserts::none(),
        )
        .await;

        connection
            .transaction_sync(|connection| {
                // Doesn't fail
                let result = IntegrationRecords::from_upsert(PullUpsertRecord::Unit(inline_init(
                    |r: &mut UnitRow| {
                        r.id = "unit".to_string();
                    },
                )))
                .integrate(connection);

                assert_eq!(result, Ok(()));

                // Fails due to referencial constraint
                let result = IntegrationRecords::from_upsert(PullUpsertRecord::Item(inline_init(
                    |r: &mut ItemRow| {
                        r.id = "item".to_string();
                        r.unit_id = Some("invalid".to_string());
                    },
                )))
                .integrate(connection);

                assert_ne!(result, Ok(()));

                Ok(()) as Result<(), ()>
            })
            .unwrap();

        // Record should exist
        assert_matches!(
            UnitRowRepository::new(&connection).find_one_by_id_option("unit"),
            Ok(Some(_))
        );

        // Record should not exist
        assert_matches!(
            ItemRowRepository::new(&connection).find_one_by_id("item"),
            Ok(None)
        );
    }

    #[actix_rt::test]
    async fn test_transaction_speed() {
        // SQLITE

        // cargo test --package service --lib data/temp.txt -- sync::translation_and_integration::test::test_transaction_speed --exact --nocapture
        // generate                        PT0S    PT0S
        // insert                          PT0.277076S     PT0.277076S
        // done insert                     PT57.376906S    PT57.099830S
        // insert transaction              PT57.376911S    PT0.000005S
        // done insert transaction         PT59.649417S    PT2.272506S

        // POSTGRES

        // cargo test --features postgres --package service --lib data/temp.txt -- sync::translation_and_integration::test::test_transaction_speed --exact --nocapture
        // generate                        PT0S    PT0S
        // insert                          PT0.281264S     PT0.281264S
        // done insert                     PT11.619679S    PT11.338415S
        // insert transaction              PT11.619684S    PT0.000005S
        // done insert transaction         PT19.922647S    PT8.302963S

        let (_, connection, _, _) = test_db::setup_all(
            "test_transaction_speed_no_tansaction",
            MockDataInserts::none(),
        )
        .await;

        let (_, connection_transact, _, _) =
            test_db::setup_all("test_transaction_speed_tansaction", MockDataInserts::none()).await;

        bench_point("generate");

        let records: Vec<UnitRow> = (0..100000)
            .into_iter()
            .map(|num| UnitRow {
                id: uuid(),
                name: uuid(),
                description: Some(uuid()),
                index: num,
            })
            .collect();

        bench_point("insert");

        let repo = UnitRowRepository::new(&connection);
        for record in records.iter() {
            repo.upsert_one(&record).unwrap();
        }

        bench_point("done insert");

        bench_point("insert transaction");

        connection_transact
            .transaction_sync(|con| {
                let repo = UnitRowRepository::new(&con);
                for record in records.iter() {
                    repo.upsert_one(&record).unwrap();
                }

                Ok(()) as Result<(), ()>
            })
            .unwrap();

        bench_point("done insert transaction");
        bench_results();
    }

    #[actix_rt::test]
    async fn test_nested_transaction_speed() {
        // SQLITE

        // cargo test --package service --lib data/temp.txt -- sync::translation_and_integration::test::test_nested_transaction_speed --exact --nocapture
        // generate                        PT0S    PT0S
        // insert transaction              PT0.267825S     PT0.267825S
        // done insert transaction         PT2.581612S     PT2.313787S
        // insert nested transaction       PT2.581617S     PT0.000005S
        // done insert nested transaction  PT5.093400S     PT2.511783S

        // POSTGRES

        // cargo test --features postgres --package service --lib data/temp.txt -- sync::translation_and_integration::test::test_nested_transaction_speed --exact --nocapture
        // generate                        PT0S    PT0S
        // insert transaction              PT0.292114S     PT0.292114S
        // done insert transaction         PT9.086202S     PT8.794088S
        // insert nested transaction       PT9.086207S     PT0.000005S
        // done insert nested transaction  PT22.342693S    PT13.256486S

        let (_, connection_transact, _, _) = test_db::setup_all(
            "test_nested_transaction_speed_transaction",
            MockDataInserts::none(),
        )
        .await;

        let (_, connection_transact_nested, _, _) = test_db::setup_all(
            "test_nested_transaction_speed_nested_transaction",
            MockDataInserts::none(),
        )
        .await;

        bench_point("generate");

        let records: Vec<UnitRow> = (0..100000)
            .into_iter()
            .map(|num| UnitRow {
                id: uuid(),
                name: uuid(),
                description: Some(uuid()),
                index: num,
            })
            .collect();

        bench_point("insert transaction");
        connection_transact
            .transaction_sync(|con| {
                let repo = UnitRowRepository::new(&con);
                for record in records.iter() {
                    repo.upsert_one(&record).unwrap();
                }

                Ok(()) as Result<(), ()>
            })
            .unwrap();

        bench_point("done insert transaction");

        bench_point("insert nested transaction");
        connection_transact_nested
            .transaction_sync(|con| {
                for record in records.iter() {
                    con.transaction_sync_etc(
                        |con| {
                            let repo = UnitRowRepository::new(&con);
                            repo.upsert_one(&record).unwrap();

                            Ok(()) as Result<(), ()>
                        },
                        false,
                    )
                    .unwrap();
                }
                Ok(()) as Result<(), ()>
            })
            .unwrap();

        bench_point("done insert nested transaction");
        bench_results();
    }

    #[actix_rt::test]
    async fn test_nested_transaction_and_errors_speed() {
        // SQLITE

        // cargo test --package service --lib data/temp.txt -- sync::translation_and_integration::test::test_nested_transaction_and_errors_speed --exact --nocapture
        // generate                        PT0S    PT0S
        // insert transaction              PT0.094401S     PT0.094401S
        // done insert transaction         PT4.992747S     PT4.898346S
        // insert nested transaction       PT4.992756S     PT0.000009S
        // done insert nested transaction  PT71.597694S    PT66.604938S

        // POSTGRES

        // cargo test --features postgres --package service --lib data/temp.txt -- sync::translation_and_integration::test::test_nested_transaction_and_errors_speed --exact --nocapture
        // generate                        PT0S    PT0S
        // insert nested transaction       PT0.129909S     PT0.129909S
        // done insert nested transaction  PT236.547470S   PT236.417561S

        let unit_row = UnitRow {
            id: uuid(),
            ..Default::default()
        };

        let (_, connection_transact, _, _) = test_db::setup_all_with_data(
            "test_nested_transaction_and_error_speed_transaction",
            MockDataInserts::none(),
            MockData {
                units: vec![unit_row.clone()],
                ..Default::default()
            },
        )
        .await;

        let (_, connection_transact_nested, _, _) = test_db::setup_all_with_data(
            "test_nested_transaction_and_error_speed_nested_transaction",
            MockDataInserts::none(),
            MockData {
                units: vec![unit_row.clone()],
                ..Default::default()
            },
        )
        .await;

        bench_point("generate");

        let records: Vec<ItemRow> = (0..100000)
            .into_iter()
            .map(|num| ItemRow {
                id: uuid(),
                // Error in half the records
                unit_id: if num % 2 == 0 {
                    Some(unit_row.id.clone())
                } else {
                    Some("does not exist".to_string())
                },
                ..Default::default()
            })
            .collect();

        // Nested transaction will error
        if cfg!(not(feature = "postgres")) {
            let mut error_count = 0;
            bench_point("insert transaction");
            connection_transact
                .transaction_sync(|con| {
                    let repo = ItemRowRepository::new(&con);
                    for record in records.iter() {
                        if repo.upsert_one(&record).is_err() {
                            error_count += 1
                        }
                    }

                    Ok(()) as Result<(), ()>
                })
                .unwrap();

            assert_eq!(error_count, 50000);
            bench_point("done insert transaction");
        }

        let mut error_count = 0;
        bench_point("insert nested transaction");
        connection_transact_nested
            .transaction_sync(|con| {
                for record in records.iter() {
                    if con
                        .transaction_sync_etc(
                            |con| {
                                let repo = ItemRowRepository::new(&con);
                                repo.upsert_one(&record)?;

                                Ok(()) as Result<(), RepositoryError>
                            },
                            false,
                        )
                        .is_err()
                    {
                        error_count += 1
                    }
                }
                Ok(()) as Result<(), ()>
            })
            .unwrap();

        assert_eq!(error_count, 50000);

        bench_point("done insert nested transaction");
        bench_results();
    }

    #[actix_rt::test]
    async fn test_transaction_fall_through() {
        // Fall through, in postgres the whole transaction fails, in sqlite just the things that was broken
        let (_, connection, _, _) =
            test_db::setup_all("test_transaction_fall_through", MockDataInserts::none()).await;

        let item1 = ItemRow {
            id: "item1".to_string(),
            ..Default::default()
        };
        let item2 = ItemRow {
            id: "item2".to_string(),
            ..Default::default()
        };

        connection
            .transaction_sync(|con| {
                let repo = ItemRowRepository::new(&con);

                repo.upsert_one(&item1).unwrap();

                let error_result = repo.upsert_one(&ItemRow {
                    id: "error".to_string(),
                    unit_id: Some("does not exit".to_string()),
                    ..Default::default()
                });

                assert!(error_result.is_err());

                let ok_or_error_result = repo.upsert_one(&item2);
                let aborted = cfg!(feature = "postgres");

                assert_eq!(ok_or_error_result.is_err(), aborted);

                Ok(()) as Result<(), ()>
            })
            .unwrap();

        // Check that item1 and item2 exist
        let repo = ItemRowRepository::new(&connection);

        let item1_from_db = repo.find_one_by_id("item1");
        let item2_from_db = repo.find_one_by_id("item2");

        if cfg!(feature = "postgres") {
            assert_eq!(item1_from_db, Ok(None));
            assert_eq!(item2_from_db, Ok(None));
        } else {
            assert_eq!(item1_from_db, Ok(Some(item1)));
            assert_eq!(item2_from_db, Ok(Some(item2)));
        }
    }
}

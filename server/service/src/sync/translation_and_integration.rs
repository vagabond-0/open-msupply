use super::sync_status::logger::{SyncLogger, SyncLoggerError, SyncStepProgress};
use super::{
    sync_buffer::SyncBuffer,
    translations::{IntegrationOperation, PullTranslateResult, SyncTranslation, SyncTranslators},
};
use crate::usize_to_u64;
use log::{debug, warn};
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
    ) -> Result<Vec<PullTranslateResult>, anyhow::Error> {
        let mut translation_results = Vec::new();

        for translator in translators.iter() {
            if !translator.should_translate_from_sync_record(sync_record) {
                continue;
            }
            let source_site_id = sync_record.source_site_id.clone();

            let mut translation_result = match sync_record.action {
                SyncAction::Upsert => translator
                    .try_translate_from_upsert_sync_record(self.connection, sync_record)?,
                SyncAction::Delete => translator
                    .try_translate_from_delete_sync_record(self.connection, sync_record)?,
                SyncAction::Merge => {
                    translator.try_translate_from_merge_sync_record(self.connection, sync_record)?
                }
            };

            // Add source_site_id to translation result if it exists in the sync buffer row
            match source_site_id {
                Some(id) => translation_result.add_source_site_id(id),
                None => {}
            }

            translation_results.push(translation_result);
        }

        Ok(translation_results)
    }

    pub(crate) fn translate_and_integrate_sync_records(
        &self,
        sync_records: Vec<SyncBufferRow>,
        translators: &Vec<Box<dyn SyncTranslation>>,
        mut logger: Option<&mut SyncLogger>,
    ) -> Result<TranslationAndIntegrationResults, RepositoryError> {
        let step_progress = SyncStepProgress::Integrate;
        let mut result = TranslationAndIntegrationResults::new();

        // Try translate
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
            let pull_translation_results =
                match self.translate_sync_record(&sync_record, translators) {
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

            let mut integration_records = Vec::new();
            let mut ignored = false;
            for pull_translation_result in pull_translation_results {
                match pull_translation_result {
                    PullTranslateResult::IntegrationOperations(mut operations) => {
                        integration_records.append(&mut operations)
                    }
                    PullTranslateResult::Ignored(ignore_message) => {
                        ignored = true;
                        self.sync_buffer.record_integration_error(
                            &sync_record,
                            &anyhow::anyhow!("Ignored: {}", ignore_message),
                        )?;
                        result.insert_error(&sync_record.table_name);

                        debug!(
                            "Ignored record: {:?} {:?} {:?}",
                            ignore_message, sync_record.record_id, sync_record.table_name
                        );
                        continue;
                    }
                    PullTranslateResult::NotMatched => {}
                }
            }

            if ignored {
                continue;
            }

            // Record translator not found error in sync buffer and in result, continue to next sync_record
            if integration_records.is_empty() {
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

            // Integrate
            let integration_result = integrate(self.connection, &integration_records);
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

impl IntegrationOperation {
    fn integrate(&self, connection: &StorageConnection) -> Result<(), RepositoryError> {
        match self {
            IntegrationOperation::Upsert(upsert, source_site_id) => {
                let cursor_id = upsert.upsert(connection)?;

                // Update the change log if we get a cursor id
                if let Some(cursor_id) = cursor_id {
                    ChangelogRepository::new(connection).set_source_site_id_and_is_sync_update(
                        cursor_id,
                        source_site_id.to_owned(),
                    )?;
                }
                Ok(())
            }

            IntegrationOperation::Delete(delete) => delete.delete(connection),
        }
    }
}

pub(crate) fn integrate(
    connection: &StorageConnection,
    integration_records: &[IntegrationOperation],
) -> Result<(), RepositoryError> {
    // Only start nested transaction if transaction is already ongoing. See integrate_and_translate_sync_buffer
    let start_nested_transaction = {
        connection
            .lock()
            .transaction_level::<RepositoryError>()
            .map_err(|e| e.to_inner_error())?
            > 0
    };

    for integration_record in integration_records.iter() {
        // Integrate every record in a sub transaction. This is mainly for Postgres where the
        // whole transaction fails when there is a DB error (not a problem in sqlite).
        if start_nested_transaction {
            connection
                .transaction_sync_etc(|sub_tx| integration_record.integrate(sub_tx), false)
                .map_err(|e| e.to_inner_error())?;
        } else {
            integration_record.integrate(connection)?;
        }
    }

    Ok(())
}

impl TranslationAndIntegrationResults {
    fn new() -> TranslationAndIntegrationResults {
        Default::default()
    }

    fn insert_error(&mut self, table_name: &str) {
        let entry = self.0.entry(table_name.to_owned()).or_default();
        entry.errors_count += 1;
    }

    fn insert_success(&mut self, table_name: &str) {
        let entry = self.0.entry(table_name.to_owned()).or_default();
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

    use crate::sync::{translation_and_integration::integrate, translations::IntegrationOperation};

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
                let result = integrate(
                    connection,
                    &[IntegrationOperation::upsert(inline_init(
                        |r: &mut UnitRow| {
                            r.id = "unit".to_string();
                        },
                    ))],
                );

                assert_eq!(result, Ok(()));

                // Fails due to referential constraint
                let result = integrate(
                    connection,
                    &[IntegrationOperation::upsert(inline_init(
                        |r: &mut ItemRow| {
                            r.id = "item".to_string();
                            r.unit_id = Some("invalid".to_string());
                        },
                    ))],
                );

                assert_ne!(result, Ok(()));

                Ok(()) as Result<(), ()>
            })
            .unwrap();

        // Record should exist
        assert_matches!(
            UnitRowRepository::new(&connection).find_one_by_id("unit"),
            Ok(Some(_))
        );

        // Record should not exist
        assert_matches!(
            ItemRowRepository::new(&connection).find_active_by_id("item"),
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
                is_active: true,
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

        // Nested transaction will error and exist postgres
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

        // Nested transaction will error
        if cfg!(feature = "postgres") {
            let mut error_count = 0;
            bench_point("insert no transaction");
            let repo = ItemRowRepository::new(&connection_transact);
            for record in records.iter() {
                if repo.upsert_one(&record).is_err() {
                    error_count += 1
                }
            }

            assert_eq!(error_count, 50000);
            bench_point("done insert no transaction");
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

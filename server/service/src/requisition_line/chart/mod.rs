use chrono::NaiveDate;
use repository::{
    requisition_row::RequisitionRowType, RepositoryError, RequisitionLine, RequisitionLineRow,
    StorageConnection,
};
mod historic_consumption;
pub use historic_consumption::*;

mod stock_evolution;
pub use stock_evolution::*;

use crate::service_provider::ServiceContext;

use super::common::check_requisition_line_exists;

#[derive(Debug, PartialEq)]
pub enum RequisitionLineChartError {
    RequisitionLineDoesNotExist,
    RequisitionLineDoesNotBelongToCurrentStore,
    NotARequestRequisition,
    // TODO not a reqest requisition
    // Internal
    DatabaseError(RepositoryError),
}
type OutError = RequisitionLineChartError;

#[derive(Debug, PartialEq, Default)]
pub struct SuggestedQuantityCalculation {
    pub average_monthly_consumption: f64,
    pub stock_on_hand: u32,
    pub minimum_stock_on_hand: f64,
    pub maximum_stock_on_hand: f64,
    pub suggested: u32,
}

#[derive(Debug, PartialEq, Default)]
pub struct ItemChart {
    pub consumption_history: Option<Vec<ConsumptionHistory>>,
    pub stock_evolution: Option<Vec<StockEvolution>>,
    pub reference_date: Option<NaiveDate>,
    pub suggested_quantity_calculation: SuggestedQuantityCalculation,
}

pub fn get_requisition_line_chart(
    ctx: &ServiceContext,
    requisition_line_id: &str,
    consumption_history_options: ConsumptionHistoryOptions,
    stock_evolution_options: StockEvolutionOptions,
) -> Result<ItemChart, OutError> {
    // Validate
    let requisition_line = validate(&ctx.connection, &ctx.store_id, requisition_line_id)?;

    let suggested_quantity_calculation =
        SuggestedQuantityCalculation::from_requisition_line(&requisition_line);

    let (expected_delivery_date, requisition_line_datetime) = match (
        &requisition_line.requisition_row.expected_delivery_date,
        &requisition_line.requisition_line_row.snapshot_datetime,
    ) {
        (Some(expected_delivery_date), Some(requisition_line_datetime)) => {
            (expected_delivery_date, requisition_line_datetime)
        }
        _ => {
            return Ok(ItemChart {
                consumption_history: None,
                stock_evolution: None,
                reference_date: None,
                suggested_quantity_calculation,
            })
        }
    };

    let RequisitionLineRow {
        item_id,
        available_stock_on_hand,
        average_monthly_consumption,
        requested_quantity,
        ..
    } = requisition_line.requisition_line_row;

    let mut consumption_history = get_historic_consumption_for_item(
        &ctx.connection,
        &ctx.store_id,
        &item_id,
        requisition_line_datetime.date(),
        consumption_history_options,
    )?;

    // Replace last consumption_history element with requisition line AMC (current AMC)
    if let Some(last) = consumption_history.last_mut() {
        last.consumption = average_monthly_consumption as u32;
        last.average_monthly_consumption = average_monthly_consumption as f64;
    }

    let StockEvolutionResult {
        mut projected_stock,
        mut historic_stock,
    } = get_stock_evolution_for_item(
        &ctx.connection,
        &ctx.store_id,
        &item_id,
        *requisition_line_datetime,
        available_stock_on_hand as u32,
        *expected_delivery_date,
        requested_quantity as u32,
        average_monthly_consumption as f64,
        stock_evolution_options,
    )?;

    historic_stock.append(&mut projected_stock);

    Ok(ItemChart {
        consumption_history: Some(consumption_history),
        stock_evolution: Some(historic_stock),
        reference_date: Some(requisition_line_datetime.date()),
        suggested_quantity_calculation,
    })
}

fn validate(
    connection: &StorageConnection,
    store_id: &str,
    requisition_line_id: &str,
) -> Result<RequisitionLine, OutError> {
    let requisition_line = check_requisition_line_exists(connection, requisition_line_id)?
        .ok_or(OutError::RequisitionLineDoesNotExist)?;

    if requisition_line.requisition_row.store_id != store_id {
        return Err(OutError::RequisitionLineDoesNotBelongToCurrentStore);
    }

    if requisition_line.requisition_row.r#type != RequisitionRowType::Request {
        return Err(OutError::NotARequestRequisition);
    }

    Ok(requisition_line)
}

impl From<RepositoryError> for OutError {
    fn from(error: RepositoryError) -> Self {
        OutError::DatabaseError(error)
    }
}

impl SuggestedQuantityCalculation {
    pub fn from_requisition_line(from: &RequisitionLine) -> Self {
        SuggestedQuantityCalculation {
            average_monthly_consumption: from.requisition_line_row.average_monthly_consumption
                as f64,
            stock_on_hand: from.requisition_line_row.available_stock_on_hand as u32,
            minimum_stock_on_hand: from.requisition_line_row.average_monthly_consumption as f64
                * from.requisition_row.min_months_of_stock,
            maximum_stock_on_hand: from.requisition_line_row.average_monthly_consumption as f64
                * from.requisition_row.max_months_of_stock as f64,
            suggested: from.requisition_line_row.suggested_quantity as u32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service_provider::ServiceProvider;
    use repository::{
        db_diesel::requisition_row::RequisitionRowType,
        mock::{
            mock_draft_response_requisition_for_update_test_line, mock_item_a, mock_name_a,
            mock_store_a, mock_store_b, MockData, MockDataInserts,
        },
        test_db::{setup_all, setup_all_with_data},
        InvoiceLineRow, InvoiceLineRowType, InvoiceRow, InvoiceRowType, NameRow,
        RequisitionLineRow, RequisitionRow, StockLineRow, StoreRow,
    };
    use util::{
        constants::NUMBER_OF_DAYS_IN_A_MONTH, date_now, inline_edit, inline_init, uuid::uuid,
    };

    type ServiceError = RequisitionLineChartError;

    #[actix_rt::test]
    async fn get_requisition_line_chart_errors() {
        let (_, _, connection_manager, _) =
            setup_all("get_requisition_line_chart_errors", MockDataInserts::all()).await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let mut context = service_provider
            .context(mock_store_a().id, "".to_string())
            .unwrap();
        let service = service_provider.requisition_line_service;

        // RequisitionLineDoesNotExist
        assert_eq!(
            service.get_requisition_line_chart(
                &context,
                "n/a",
                ConsumptionHistoryOptions::default(),
                StockEvolutionOptions::default(),
            ),
            Err(ServiceError::RequisitionLineDoesNotExist)
        );

        let test_line = mock_draft_response_requisition_for_update_test_line();

        // NotARequestRequisition
        assert_eq!(
            service.get_requisition_line_chart(
                &context,
                &test_line.id,
                ConsumptionHistoryOptions::default(),
                StockEvolutionOptions::default(),
            ),
            Err(ServiceError::NotARequestRequisition)
        );

        // RequisitionLineDoesNotBelongToCurrentStore
        context.store_id = mock_store_b().id;
        assert_eq!(
            service.get_requisition_line_chart(
                &context,
                &test_line.id,
                ConsumptionHistoryOptions::default(),
                StockEvolutionOptions::default(),
            ),
            Err(ServiceError::RequisitionLineDoesNotBelongToCurrentStore)
        );
    }

    #[actix_rt::test]
    async fn get_requisition_line_chart_consumption() {
        fn name() -> NameRow {
            inline_init(|r: &mut NameRow| {
                r.id = "name".to_string();
            })
        }

        fn store() -> StoreRow {
            inline_init(|s: &mut StoreRow| {
                s.id = "store".to_string();
                s.name_id = name().id;
                s.code = "n/a".to_string();
            })
        }

        fn requisition() -> RequisitionRow {
            inline_init(|r: &mut RequisitionRow| {
                r.id = "requisition".to_string();
                r.store_id = store().id;
                r.name_id = mock_name_a().id;
                r.expected_delivery_date = Some(date_now());
                r.r#type = RequisitionRowType::Request;
            })
        }

        fn requisition_line() -> RequisitionLineRow {
            inline_init(|r: &mut RequisitionLineRow| {
                r.id = "requisition_line".to_string();
                r.requisition_id = requisition().id;
                r.item_id = mock_item_a().id;
                r.snapshot_datetime = Some(NaiveDate::from_ymd(2021, 01, 02).and_hms(0, 0, 0));
                r.average_monthly_consumption = 333;
            })
        }

        fn consumption_point() -> MockData {
            let invoice_id = uuid();
            inline_init(|r: &mut MockData| {
                r.invoices = vec![inline_init(|r: &mut InvoiceRow| {
                    r.id = invoice_id.clone();
                    r.store_id = store().id;
                    r.name_id = mock_name_a().id;
                    r.r#type = InvoiceRowType::OutboundShipment;
                })];
                r.invoice_lines = vec![inline_init(|r: &mut InvoiceLineRow| {
                    r.id = format!("{}line", invoice_id);
                    r.invoice_id = invoice_id.clone();
                    r.item_id = mock_item_a().id;
                    r.r#type = InvoiceLineRowType::StockOut;
                    r.stock_line_id = Some(format!("{}stock_line", invoice_id));
                    r.pack_size = 1;
                })];
                r.stock_lines = vec![inline_init(|r: &mut StockLineRow| {
                    r.id = format!("{}stock_line", invoice_id);
                    r.store_id = store().id;
                    r.item_id = mock_item_a().id;
                    r.pack_size = 1;
                })];
            })
        }

        let (_, _, connection_manager, _) = setup_all_with_data(
            "get_requisition_line_chart_consumption",
            MockDataInserts::all(),
            inline_init(|r: &mut MockData| {
                r.names = vec![name()];
                r.stores = vec![store()];
                r.requisitions = vec![requisition()];
                r.requisition_lines = vec![requisition_line()];
            })
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2021, 01, 02).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 20;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 12, 4).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 10;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 11, 30).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 30;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 10, 10).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 40;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 10, 11).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 5;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 09, 25).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 5;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 09, 10).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 20;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 08, 07).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 15;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 07, 03).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 40;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2020, 06, 20).and_hms(0, 0, 0));
                u.invoice_lines[0].number_of_packs = 30;
                u
            })),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider
            .context(store().id, "".to_string())
            .unwrap();
        let service = service_provider.requisition_line_service;

        let result = service
            .get_requisition_line_chart(
                &context,
                &requisition_line().id,
                ConsumptionHistoryOptions {
                    amc_lookback_months: 5,
                    number_of_data_points: 3,
                },
                StockEvolutionOptions::default(),
            )
            .unwrap();

        assert_eq!(
            result.consumption_history.unwrap(),
            vec![
                ConsumptionHistory {
                    // 2020-11-01 to 2020-11-30
                    consumption: 30,
                    // 2020-07-01 to 2020-11-30
                    average_monthly_consumption: (30 + 40 + 5 + 5 + 20 + 15 + 40) as f64
                        / (NaiveDate::from_ymd(2020, 11, 30) - NaiveDate::from_ymd(2020, 07, 01))
                            .num_days() as f64
                        * NUMBER_OF_DAYS_IN_A_MONTH,
                    date: NaiveDate::from_ymd(2020, 11, 30)
                },
                ConsumptionHistory {
                    // 2020-12-01 to 2020-12-31
                    consumption: 10,
                    // 2020-08-01 to 2020-12-31
                    average_monthly_consumption: (10 + 30 + 40 + 5 + 5 + 20 + 15) as f64
                        / (NaiveDate::from_ymd(2020, 12, 31) - NaiveDate::from_ymd(2020, 08, 01))
                            .num_days() as f64
                        * NUMBER_OF_DAYS_IN_A_MONTH,
                    date: NaiveDate::from_ymd(2020, 12, 31)
                },
                ConsumptionHistory {
                    // This is populated by requisition line amc
                    consumption: 333,
                    average_monthly_consumption: 333.0,
                    date: NaiveDate::from_ymd(2021, 01, 31)
                },
            ]
        );
    }

    #[actix_rt::test]
    async fn get_requisition_line_chart_stock_evolution() {
        fn name() -> NameRow {
            inline_init(|r: &mut NameRow| {
                r.id = "name".to_string();
            })
        }

        fn store() -> StoreRow {
            inline_init(|s: &mut StoreRow| {
                s.id = "store".to_string();
                s.name_id = name().id;
                s.code = "n/a".to_string();
            })
        }

        fn requisition() -> RequisitionRow {
            inline_init(|r: &mut RequisitionRow| {
                r.id = "requisition".to_string();
                r.store_id = store().id;
                r.name_id = mock_name_a().id;
                r.expected_delivery_date = Some(NaiveDate::from_ymd(2021, 1, 5));
                r.r#type = RequisitionRowType::Request;
            })
        }

        fn requisition_line() -> RequisitionLineRow {
            inline_init(|r: &mut RequisitionLineRow| {
                r.id = "requisition_line".to_string();
                r.requisition_id = requisition().id;
                r.item_id = mock_item_a().id;
                r.snapshot_datetime = Some(NaiveDate::from_ymd(2021, 1, 2).and_hms(12, 10, 11));
                r.average_monthly_consumption = 25 * NUMBER_OF_DAYS_IN_A_MONTH as i32;
                r.available_stock_on_hand = 30;
                r.requested_quantity = 100;
            })
        }

        fn consumption_point() -> MockData {
            let invoice_id = uuid();
            inline_init(|r: &mut MockData| {
                r.invoices = vec![inline_init(|r: &mut InvoiceRow| {
                    r.id = invoice_id.clone();
                    r.store_id = store().id;
                    r.name_id = mock_name_a().id;
                    r.r#type = InvoiceRowType::OutboundShipment;
                })];
                r.invoice_lines = vec![inline_init(|r: &mut InvoiceLineRow| {
                    r.id = format!("{}line", invoice_id);
                    r.invoice_id = invoice_id.clone();
                    r.item_id = mock_item_a().id;
                    r.r#type = InvoiceLineRowType::StockOut;
                    r.stock_line_id = Some(format!("{}stock_line", invoice_id));
                    r.pack_size = 1;
                })];
                r.stock_lines = vec![inline_init(|r: &mut StockLineRow| {
                    r.id = format!("{}stock_line", invoice_id);
                    r.store_id = store().id;
                    r.item_id = mock_item_a().id;
                    r.pack_size = 1;
                })];
            })
        }

        let (_, _, connection_manager, _) = setup_all_with_data(
            "get_requisition_line_chart_stock_evolution",
            MockDataInserts::all(),
            inline_init(|r: &mut MockData| {
                r.names = vec![name()];
                r.stores = vec![store()];
                r.requisitions = vec![requisition()];
                r.requisition_lines = vec![requisition_line()];
            })
            .join(inline_edit(&consumption_point(), |mut u| {
                // + 10 (Inbound Shipment)
                u.invoices[0].delivered_datetime =
                    Some(NaiveDate::from_ymd(2021, 1, 2).and_hms(10, 0, 0));
                u.invoices[0].r#type = InvoiceRowType::InboundShipment;
                u.invoice_lines[0].number_of_packs = 10;
                u.invoice_lines[0].r#type = InvoiceLineRowType::StockIn;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                // - 20 (Outbound Shipment)
                u.invoices[0].picked_datetime =
                    Some(NaiveDate::from_ymd(2021, 1, 2).and_hms(7, 0, 0));
                u.invoice_lines[0].number_of_packs = 20;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                // + 15 (Inventory Adjustment)
                u.invoices[0].verified_datetime =
                    Some(NaiveDate::from_ymd(2021, 1, 1).and_hms(2, 0, 0));
                u.invoices[0].r#type = InvoiceRowType::InventoryAdjustment;
                u.invoice_lines[0].number_of_packs = 15;
                u.invoice_lines[0].r#type = InvoiceLineRowType::StockIn;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                // + 7 (Inbound Shipment)
                u.invoices[0].delivered_datetime =
                    Some(NaiveDate::from_ymd(2021, 1, 1).and_hms(2, 0, 0));
                u.invoices[0].r#type = InvoiceRowType::InboundShipment;
                u.invoice_lines[0].number_of_packs = 7;
                u.invoice_lines[0].r#type = InvoiceLineRowType::StockIn;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                // - 11 (Inventory Adjustment)
                u.invoices[0].verified_datetime =
                    Some(NaiveDate::from_ymd(2021, 1, 1).and_hms(2, 0, 0));
                u.invoices[0].r#type = InvoiceRowType::InventoryAdjustment;
                u.invoice_lines[0].number_of_packs = 11;
                u.invoice_lines[0].r#type = InvoiceLineRowType::StockOut;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                // Not Counted
                u.invoices[0].delivered_datetime =
                    Some(NaiveDate::from_ymd(2021, 1, 3).and_hms(2, 0, 0));
                u.invoices[0].r#type = InvoiceRowType::InboundShipment;
                u.invoice_lines[0].number_of_packs = 10;
                u.invoice_lines[0].r#type = InvoiceLineRowType::StockIn;
                u
            }))
            .join(inline_edit(&consumption_point(), |mut u| {
                // Not Counted
                u.invoices[0].verified_datetime =
                    Some(NaiveDate::from_ymd(2020, 12, 31).and_hms(2, 0, 0));
                u.invoices[0].r#type = InvoiceRowType::InventoryAdjustment;
                u.invoice_lines[0].number_of_packs = 11;
                u.invoice_lines[0].r#type = InvoiceLineRowType::StockOut;
                u
            })),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider
            .context(store().id, "".to_string())
            .unwrap();
        let service = service_provider.requisition_line_service;

        let result = service
            .get_requisition_line_chart(
                &context,
                &requisition_line().id,
                ConsumptionHistoryOptions::default(),
                StockEvolutionOptions {
                    number_of_historic_data_points: 3,
                    number_of_projected_data_points: 4,
                },
            )
            .unwrap();

        assert_eq!(
            result.stock_evolution.unwrap(),
            vec![
                // Historic
                StockEvolution {
                    date: NaiveDate::from_ymd(2020, 12, 31),
                    quantity: 29.0 // (40) - 15 - 7 + 11 = (29)
                },
                StockEvolution {
                    date: NaiveDate::from_ymd(2021, 1, 1),
                    quantity: 40.0 // 30 - 10 + 20 = (40)
                },
                StockEvolution {
                    date: NaiveDate::from_ymd(2021, 1, 2),
                    quantity: 30.0 // initial
                },
                // Projected
                StockEvolution {
                    date: NaiveDate::from_ymd(2021, 1, 3),
                    quantity: 5.0 // 30 - 25 - 5
                },
                StockEvolution {
                    date: NaiveDate::from_ymd(2021, 1, 4),
                    quantity: 0.0 // (5) - 25 = -something, but we set to (0)
                },
                StockEvolution {
                    date: NaiveDate::from_ymd(2021, 1, 5),
                    quantity: 75.0 // (0) - 25 + 50 = (75), adding suggested
                },
                StockEvolution {
                    date: NaiveDate::from_ymd(2021, 1, 6),
                    quantity: 50.0 // (75) - 25 = 50.0
                },
            ]
        );
    }
}

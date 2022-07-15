use crate::{invoice_line::query::get_invoice_line, service_provider::ServiceContext, WithDBError};
use chrono::NaiveDate;
use repository::{
    InvoiceLine, InvoiceLineRowRepository, InvoiceRowRepository, RepositoryError,
    StockLineRowRepository,
};

mod generate;
mod validate;

use generate::generate;
use validate::validate;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InsertInboundShipmentLine {
    pub id: String,
    pub invoice_id: String,
    pub item_id: String,
    pub location_id: Option<String>,
    pub pack_size: u32,
    pub batch: Option<String>,
    pub cost_price_per_pack: f64,
    pub sell_price_per_pack: f64,
    pub expiry_date: Option<NaiveDate>,
    pub number_of_packs: u32,
    pub total_before_tax: f64,
    pub total_after_tax: f64,
    pub tax: Option<f64>,
}

type OutError = InsertInboundShipmentLineError;

pub fn insert_inbound_shipment_line(
    ctx: &ServiceContext,
    _store_id: &str,
    user_id: &str,
    input: InsertInboundShipmentLine,
) -> Result<InvoiceLine, OutError> {
    let new_line = ctx
        .connection
        .transaction_sync(|connection| {
            let (item, invoice) = validate(&input, &connection)?;
            let (invoice_row_option, new_line, new_batch_option) =
                generate(user_id, input, item, invoice);

            if let Some(new_batch) = new_batch_option {
                StockLineRowRepository::new(&connection).upsert_one(&new_batch)?;
            }
            InvoiceLineRowRepository::new(&connection).upsert_one(&new_line)?;

            if let Some(invoice_row) = invoice_row_option {
                InvoiceRowRepository::new(&connection).upsert_one(&invoice_row)?;
            }

            get_invoice_line(ctx, &new_line.id)
                .map_err(|error| OutError::DatabaseError(error))?
                .ok_or(OutError::NewlyCreatedLineDoesNotExist)
        })
        .map_err(|error| error.to_inner_error())?;
    Ok(new_line)
}

#[derive(Debug, PartialEq)]
pub enum InsertInboundShipmentLineError {
    LineAlreadyExists,
    DatabaseError(RepositoryError),
    InvoiceDoesNotExist,
    NotAnInboundShipment,
    NotThisStoreInvoice,
    CannotEditFinalised,
    LocationDoesNotExist,
    ItemNotFound,
    PackSizeBelowOne,
    NumberOfPacksBelowOne,
    NewlyCreatedLineDoesNotExist,
}

impl From<RepositoryError> for InsertInboundShipmentLineError {
    fn from(error: RepositoryError) -> Self {
        InsertInboundShipmentLineError::DatabaseError(error)
    }
}

impl<ERR> From<WithDBError<ERR>> for InsertInboundShipmentLineError
where
    ERR: Into<InsertInboundShipmentLineError>,
{
    fn from(result: WithDBError<ERR>) -> Self {
        match result {
            WithDBError::DatabaseError(error) => error.into(),
            WithDBError::Error(error) => error.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use repository::{
        mock::{
            mock_inbound_shipment_a_invoice_lines, mock_inbound_shipment_c_invoice_lines,
            mock_item_a, mock_outbound_shipment_c_invoice_lines, mock_store_a, mock_user_account_a,
            MockDataInserts,
        },
        test_db::setup_all,
        InvoiceLineRowRepository,
    };
    use util::{inline_edit, inline_init};

    use crate::{
        invoice_line::inbound_shipment_line::{
            insert::InsertInboundShipmentLine, InsertInboundShipmentLineError as ServiceError,
        },
        service_provider::ServiceProvider,
    };

    #[actix_rt::test]
    async fn insert_inbound_shipment_line_errors() {
        let (_, _, connection_manager, _) = setup_all(
            "insert_inbound_shipment_line_errors",
            MockDataInserts::all(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.context().unwrap();
        let service = service_provider.invoice_line_service;

        // LineAlreadyExists
        assert_eq!(
            service.insert_inbound_shipment_line(
                &context,
                &mock_store_a().id,
                &mock_user_account_a().id,
                inline_init(|r: &mut InsertInboundShipmentLine| {
                    r.id = mock_inbound_shipment_a_invoice_lines()[0].id.clone();
                    r.invoice_id = mock_inbound_shipment_c_invoice_lines()[0]
                        .invoice_id
                        .clone();
                }),
            ),
            Err(ServiceError::LineAlreadyExists)
        );

        // InvoiceDoesNotExist
        assert_eq!(
            service.insert_inbound_shipment_line(
                &context,
                &mock_store_a().id,
                &mock_user_account_a().id,
                inline_init(|r: &mut InsertInboundShipmentLine| {
                    r.id = "new invoice line id".to_string();
                    r.invoice_id = "new invoice id".to_string();
                    r.item_id = mock_item_a().id.clone();
                    r.pack_size = 1;
                    r.number_of_packs = 1;
                }),
            ),
            Err(ServiceError::InvoiceDoesNotExist)
        );

        // NotAnInboundShipment
        assert_eq!(
            service.insert_inbound_shipment_line(
                &context,
                &mock_store_a().id,
                &mock_user_account_a().id,
                inline_init(|r: &mut InsertInboundShipmentLine| {
                    r.id = "new invoice line id".to_string();
                    r.invoice_id = mock_outbound_shipment_c_invoice_lines()[0]
                        .invoice_id
                        .clone();
                    r.item_id = mock_item_a().id.clone();
                    r.pack_size = 1;
                    r.number_of_packs = 1;
                }),
            ),
            Err(ServiceError::NotAnInboundShipment)
        );

        // LocationDoesNotExist
        assert_eq!(
            service.insert_inbound_shipment_line(
                &context,
                &"new store id".to_string(),
                &mock_user_account_a().id,
                inline_init(|r: &mut InsertInboundShipmentLine| {
                    r.id = "new invoice line id".to_string();
                    r.invoice_id = mock_inbound_shipment_c_invoice_lines()[0]
                        .invoice_id
                        .clone();
                    r.location_id = Some("invalid".to_string());
                    r.item_id = mock_item_a().id.clone();
                    r.pack_size = 1;
                    r.number_of_packs = 1;
                }),
            ),
            Err(ServiceError::LocationDoesNotExist)
        );

        // ItemNotFound
        assert_eq!(
            service.insert_inbound_shipment_line(
                &context,
                &mock_store_a().id,
                &mock_user_account_a().id,
                inline_init(|r: &mut InsertInboundShipmentLine| {
                    r.id = "new invoice line id".to_string();
                    r.invoice_id = mock_inbound_shipment_c_invoice_lines()[0]
                        .invoice_id
                        .clone();
                    r.item_id = "invalid".to_string();
                    r.pack_size = 1;
                    r.number_of_packs = 1;
                }),
            ),
            Err(ServiceError::ItemNotFound)
        );

        // PackSizeBelowOne
        assert_eq!(
            service.insert_inbound_shipment_line(
                &context,
                &mock_store_a().id,
                &mock_user_account_a().id,
                inline_init(|r: &mut InsertInboundShipmentLine| {
                    r.id = "new invoice line id".to_string();
                    r.invoice_id = mock_inbound_shipment_c_invoice_lines()[0]
                        .invoice_id
                        .clone();
                    r.item_id = mock_item_a().id.clone();
                    r.pack_size = 0;
                    r.number_of_packs = 1;
                }),
            ),
            Err(ServiceError::PackSizeBelowOne)
        );

        // NumberOfPacksBelowOne
        assert_eq!(
            service.insert_inbound_shipment_line(
                &context,
                &mock_store_a().id,
                &mock_user_account_a().id,
                inline_init(|r: &mut InsertInboundShipmentLine| {
                    r.id = "new invoice line id".to_string();
                    r.invoice_id = mock_inbound_shipment_c_invoice_lines()[0]
                        .invoice_id
                        .clone();
                    r.item_id = mock_item_a().id.clone();
                    r.pack_size = 1;
                    r.number_of_packs = 0;
                }),
            ),
            Err(ServiceError::NumberOfPacksBelowOne)
        );

        //TODO DatabaseError, NewlyCreatedLineDoesNotExist
    }

    #[actix_rt::test]
    async fn insert_inbound_shipment_line_success() {
        let (_, connection, connection_manager, _) = setup_all(
            "insert_inbound_shipment_line_success",
            MockDataInserts::all(),
        )
        .await;

        let service_provider = ServiceProvider::new(connection_manager, "app_data");
        let context = service_provider.context().unwrap();
        let service = service_provider.invoice_line_service;

        service
            .insert_inbound_shipment_line(
                &context,
                &mock_store_a().id,
                &mock_user_account_a().id,
                inline_init(|r: &mut InsertInboundShipmentLine| {
                    r.id = "new invoice line id".to_string();
                    r.invoice_id = mock_inbound_shipment_c_invoice_lines()[0]
                        .invoice_id
                        .clone();
                    r.item_id = mock_item_a().id.clone();
                    r.pack_size = 1;
                    r.number_of_packs = 1;
                }),
            )
            .unwrap();

        let inbound_line = InvoiceLineRowRepository::new(&connection)
            .find_one_by_id("new invoice line id")
            .unwrap();

        assert_eq!(
            inbound_line,
            inline_edit(&inbound_line, |mut u| {
                u.id = "new invoice line id".to_string();
                u.item_id = mock_item_a().id.clone();
                u.pack_size = 1;
                u.number_of_packs = 1;
                u
            })
        );
    }
}

use chrono::Utc;

use repository::LocationMovementRow;
use repository::{
    InvoiceLineRow, InvoiceLineRowRepository, InvoiceRow, InvoiceRowStatus, StockLineRow,
    StorageConnection,
};
use util::uuid::uuid;

use super::{UpdateInboundReturn, UpdateInboundReturnError, UpdateInboundReturnStatus};

pub struct LineAndStockLine {
    pub stock_line: StockLineRow,
    pub line: InvoiceLineRow,
}

pub(crate) struct GenerateResult {
    pub(crate) batches_to_update: Option<Vec<LineAndStockLine>>,
    pub(crate) updated_return: InvoiceRow,
    pub(crate) location_movements: Option<Vec<LocationMovementRow>>,
}

pub(crate) fn generate(
    connection: &StorageConnection,
    store_id: &str,
    user_id: &str,
    existing_return: InvoiceRow,
    patch: UpdateInboundReturn,
) -> Result<GenerateResult, UpdateInboundReturnError> {
    let mut updated_return = existing_return.clone();

    updated_return.user_id = Some(user_id.to_string());
    // update_invoice.comment = patch.comment.or(update_invoice.comment);
    // update_invoice.on_hold = patch.on_hold.unwrap_or(update_invoice.on_hold);
    // update_invoice.colour = patch.colour.or(update_invoice.colour);

    set_new_status_datetime(&mut updated_return, &patch);

    if let Some(status) = patch.status.clone() {
        updated_return.status = status.as_invoice_row_status()
    }

    let should_create_batches = should_create_batches(&existing_return.status, &patch);

    let batches_to_update = if should_create_batches {
        Some(generate_lines_and_stock_lines(
            connection,
            &updated_return.store_id,
            &updated_return.id,
            &updated_return.name_link_id,
        )?)
    } else {
        None
    };

    let location_movements = if let Some(batches) = &batches_to_update {
        let generate_movement = batches
            .iter()
            .filter_map(|batch| match batch.line.location_id {
                Some(_) => Some(generate_location_movements(store_id.to_owned(), batch)),
                None => None,
            })
            .collect();

        Some(generate_movement)
    } else {
        None
    };

    Ok(GenerateResult {
        batches_to_update,
        updated_return,
        location_movements,
    })
}

fn changed_status(
    status: Option<UpdateInboundReturnStatus>,
    existing_status: &InvoiceRowStatus,
) -> Option<UpdateInboundReturnStatus> {
    let new_status = match status {
        Some(status) => status,
        None => return None, // Status is not changing
    };

    if &new_status.as_invoice_row_status() == existing_status {
        // The invoice already has this status, there's nothing to do.
        return None;
    }

    Some(new_status)
}

pub fn should_create_batches(
    existing_status: &InvoiceRowStatus,
    patch: &UpdateInboundReturn,
) -> bool {
    let new_status = match changed_status(patch.status.to_owned(), existing_status) {
        Some(status) => status,
        None => return false, // There's no status to update
    };

    match (existing_status, new_status) {
        (
            // From New/Picked/Shipped to Delivered
            InvoiceRowStatus::New | InvoiceRowStatus::Picked | InvoiceRowStatus::Shipped,
            UpdateInboundReturnStatus::Delivered,
        ) => true,
        _ => false,
    }
}

fn set_new_status_datetime(inbound_return: &mut InvoiceRow, patch: &UpdateInboundReturn) {
    let new_status = match changed_status(patch.status.to_owned(), &inbound_return.status) {
        Some(status) => status,
        None => return, // There's no status to update
    };

    let current_datetime = Utc::now().naive_utc();

    // TODO: is this right??
    match (&inbound_return.status, new_status) {
        // From New/Picked/Shipped to Delivered
        (
            InvoiceRowStatus::New | InvoiceRowStatus::Picked | InvoiceRowStatus::Shipped,
            UpdateInboundReturnStatus::Delivered,
        ) => {
            inbound_return.delivered_datetime = Some(current_datetime);
        }

        // From New to Verified
        (InvoiceRowStatus::New, UpdateInboundReturnStatus::Verified)
        | (InvoiceRowStatus::Picked, UpdateInboundReturnStatus::Verified)
        | (InvoiceRowStatus::Shipped, UpdateInboundReturnStatus::Verified) => {
            inbound_return.delivered_datetime = Some(current_datetime);
            inbound_return.verified_datetime = Some(current_datetime);
        }

        // From Delivered to Verified
        (InvoiceRowStatus::Delivered, UpdateInboundReturnStatus::Verified) => {
            inbound_return.verified_datetime = Some(current_datetime);
        }
        _ => {}
    }
}

pub fn generate_lines_and_stock_lines(
    connection: &StorageConnection,
    store_id: &str,
    inbound_return_id: &str,
    supplier_id: &str,
) -> Result<Vec<LineAndStockLine>, UpdateInboundReturnError> {
    let return_lines =
        InvoiceLineRowRepository::new(connection).find_many_by_invoice_id(inbound_return_id)?;
    let mut result = Vec::new();

    for line in return_lines.into_iter() {
        let mut return_line = line.clone();

        let stock_line_id = return_line.stock_line_id.unwrap_or(uuid());
        return_line.stock_line_id = Some(stock_line_id.clone());

        let InvoiceLineRow {
            id: _,
            invoice_id: _,
            item_link_id,
            item_name: _,
            item_code: _,
            stock_line_id: _,
            location_id,
            batch,
            expiry_date,
            pack_size,
            cost_price_per_pack,
            sell_price_per_pack,
            total_before_tax: _,
            total_after_tax: _,
            tax: _,
            r#type: _,
            number_of_packs,
            note,
            inventory_adjustment_reason_id: _,
            return_reason_id: _,
            foreign_currency_price_before_tax: _,
        }: InvoiceLineRow = line;

        if number_of_packs > 0.0 {
            let stock_line = StockLineRow {
                id: stock_line_id,
                item_link_id,
                store_id: store_id.to_string(),
                location_id,
                batch,
                pack_size,
                cost_price_per_pack,
                sell_price_per_pack,
                available_number_of_packs: number_of_packs,
                total_number_of_packs: number_of_packs,
                expiry_date,
                on_hold: false,
                note,
                supplier_link_id: Some(supplier_id.to_string()),
                barcode_id: None,
            };
            result.push(LineAndStockLine {
                line: return_line,
                stock_line,
            });
        }
    }
    Ok(result)
}

pub fn generate_location_movements(
    store_id: String,
    batch: &LineAndStockLine,
) -> LocationMovementRow {
    LocationMovementRow {
        id: uuid(),
        store_id,
        stock_line_id: batch.stock_line.id.clone(),
        location_id: batch.line.location_id.clone(),
        enter_datetime: Some(Utc::now().naive_utc()),
        exit_datetime: None,
    }
}

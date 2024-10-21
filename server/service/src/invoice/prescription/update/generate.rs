use chrono::{NaiveDateTime, Utc};

use repository::{
    EqualFilter, InvoiceLineFilter, InvoiceLineRepository, InvoiceLineRow, InvoiceRow,
    InvoiceStatus, RepositoryError, StockLineRow, StorageConnection,
};

use crate::invoice::common::{
    generate_batches_total_number_of_packs_update, InvoiceLineHasNoStockLine,
};

use super::{UpdatePrescription, UpdatePrescriptionError, UpdatePrescriptionStatus};

pub(crate) struct GenerateResult {
    pub(crate) batches_to_update: Option<Vec<StockLineRow>>,
    pub(crate) update_invoice: InvoiceRow,
    pub(crate) lines_to_trim: Option<Vec<InvoiceLineRow>>,
}

pub(crate) fn generate(
    existing_invoice: InvoiceRow,
    UpdatePrescription {
        id: _,
        status: input_status,
        patient_id: input_patient_id,
        clinician_id: input_clinician_id,
        comment: input_comment,
        colour: input_colour,
        backdated_datetime: input_backdated_datetime,
    }: UpdatePrescription,
    connection: &StorageConnection,
) -> Result<GenerateResult, UpdatePrescriptionError> {
    let should_update_batches_total_number_of_packs =
        should_update_batches_total_number_of_packs(&existing_invoice, &input_status);
    let mut update_invoice = existing_invoice.clone();

    set_new_status_datetime(&mut update_invoice, &input_status);

    let backdated_datetime = backdate_status_datetimes(
        &mut update_invoice,
        &existing_invoice,
        input_backdated_datetime,
    );

    update_invoice.name_link_id = input_patient_id.unwrap_or(update_invoice.name_link_id);
    update_invoice.clinician_link_id = input_clinician_id.or(update_invoice.clinician_link_id);
    update_invoice.comment = input_comment.or(update_invoice.comment);
    update_invoice.colour = input_colour.or(update_invoice.colour);
    update_invoice.backdated_datetime = backdated_datetime;

    if let Some(status) = input_status.clone() {
        update_invoice.status = status.full_status()
    }

    let batches_to_update = if should_update_batches_total_number_of_packs {
        Some(
            generate_batches_total_number_of_packs_update(&update_invoice.id, connection).map_err(
                |e| match e {
                    InvoiceLineHasNoStockLine::InvoiceLineHasNoStockLine(line) => {
                        UpdatePrescriptionError::InvoiceLineHasNoStockLine(line)
                    }
                    InvoiceLineHasNoStockLine::DatabaseError(e) => {
                        UpdatePrescriptionError::DatabaseError(e)
                    }
                },
            )?,
        )
    } else {
        None
    };

    let lines_to_trim = lines_to_trim(connection, &existing_invoice, &input_status)?;

    Ok(GenerateResult {
        batches_to_update,
        update_invoice,
        lines_to_trim,
    })
}

fn should_update_batches_total_number_of_packs(
    invoice: &InvoiceRow,
    status: &Option<UpdatePrescriptionStatus>,
) -> bool {
    if let Some(new_invoice_status) = UpdatePrescriptionStatus::full_status_option(status) {
        let invoice_status_index = invoice.status.index();
        let new_invoice_status_index = new_invoice_status.index();

        new_invoice_status_index >= InvoiceStatus::Picked.index()
            && invoice_status_index < InvoiceStatus::Picked.index()
    } else {
        false
    }
}

enum BackdateAction {
    NoUpdate,
    // Will also rest to now if new input is None
    // or if new input mutation backdated_datetime input is later then created_datetime
    ResetToNow,
    Backdate(NaiveDateTime),
}
// Replace datestimes that are not null with backdated_datetime
fn backdate_status_datetimes(
    invoice: &mut InvoiceRow,
    existing_invoice: &InvoiceRow,
    input_backdated_datetime: Option<NaiveDateTime>,
) -> Option<NaiveDateTime> {
    use BackdateAction::*;

    let action = match (
        existing_invoice.backdated_datetime,
        input_backdated_datetime,
    ) {
        (None, None) => NoUpdate,
        (Some(_), None) => ResetToNow,
        (Some(_), Some(input)) | (None, Some(input)) => {
            if input > existing_invoice.created_datetime {
                ResetToNow
            } else {
                Backdate(input)
            }
        }
    };

    let update_to_datetime = match action {
        NoUpdate => return None,
        ResetToNow => Utc::now().naive_utc(),
        Backdate(update_to_datetime) => update_to_datetime,
    };
    invoice.allocated_datetime = invoice.allocated_datetime.map(|_| update_to_datetime);
    invoice.picked_datetime = invoice.picked_datetime.map(|_| update_to_datetime);
    invoice.verified_datetime = invoice.verified_datetime.map(|_| update_to_datetime);

    Some(update_to_datetime)
}

fn set_new_status_datetime(invoice: &mut InvoiceRow, status: &Option<UpdatePrescriptionStatus>) {
    let new_status = match status {
        Some(status) => status,
        None => return,
    };

    if new_status.full_status() == invoice.status {
        return;
    }

    let current_datetime = Utc::now().naive_utc();

    match (&invoice.status, new_status) {
        (InvoiceStatus::Verified, _) => {}
        (InvoiceStatus::New, UpdatePrescriptionStatus::Verified) => {
            invoice.picked_datetime = Some(current_datetime);
            invoice.verified_datetime = Some(current_datetime)
        }
        (InvoiceStatus::New, UpdatePrescriptionStatus::Picked) => {
            invoice.picked_datetime = Some(current_datetime);
        }
        (InvoiceStatus::Picked, UpdatePrescriptionStatus::Verified) => {
            invoice.verified_datetime = Some(current_datetime)
        }
        _ => {}
    }
}

// If status changed to verified, remove empty lines
fn lines_to_trim(
    connection: &StorageConnection,
    invoice: &InvoiceRow,
    status: &Option<UpdatePrescriptionStatus>,
) -> Result<Option<Vec<InvoiceLineRow>>, RepositoryError> {
    // Status sequence for outbound shipment: New, Picked, Verified
    if invoice.status == InvoiceStatus::Verified {
        return Ok(None);
    }

    let new_prescription_status = match UpdatePrescriptionStatus::full_status_option(status) {
        Some(new_prescription_status) => new_prescription_status,
        None => return Ok(None),
    };

    if new_prescription_status != InvoiceStatus::Verified {
        return Ok(None);
    }

    // If new status is Verified and previous invoice status is Picked
    // add all lines to be deleted
    let empty_lines = InvoiceLineRepository::new(connection).query_by_filter(
        InvoiceLineFilter::new()
            .invoice_id(EqualFilter::equal_to(&invoice.id))
            .number_of_packs(EqualFilter::equal_to_f64(0.0)),
    )?;

    if empty_lines.is_empty() {
        return Ok(None);
    }

    let invoice_line_rows = empty_lines
        .into_iter()
        .map(|l| l.invoice_line_row)
        .collect();
    Ok(Some(invoice_line_rows))
}

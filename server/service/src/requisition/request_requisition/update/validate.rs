use super::{OutError, UpdateRequestRequisition};
use crate::{
    requisition::common::{
        check_emergency_order_within_max_items_limit, check_requisition_row_exists,
        OrderTypeNotFoundError,
    },
    validate::{check_other_party, CheckOtherPartyType, OtherPartyErrors},
};
use repository::{
    requisition_row::{RequisitionRow, RequisitionStatus, RequisitionType},
    EqualFilter, RequisitionLineFilter, RequisitionLineRepository, StorageConnection,
};

pub fn validate(
    connection: &StorageConnection,
    store_id: &str,
    input: &UpdateRequestRequisition,
) -> Result<(RequisitionRow, bool), OutError> {
    let requisition_row = check_requisition_row_exists(connection, &input.id)?
        .ok_or(OutError::RequisitionDoesNotExist)?;
    let requisition_lines = RequisitionLineRepository::new(connection).query_by_filter(
        RequisitionLineFilter::new().requisition_id(EqualFilter::equal_to(&requisition_row.id)),
    )?;
    let status_changed = input.status.is_some();

    if requisition_row.program_id.is_some()
        && (input.other_party_id.is_some()
            || input.min_months_of_stock.is_some()
            || input.max_months_of_stock.is_some())
    {
        return Err(OutError::CannotEditProgramRequisitionInformation);
    }

    if requisition_row.store_id != store_id {
        return Err(OutError::NotThisStoreRequisition);
    }

    if requisition_row.r#type != RequisitionType::Request {
        return Err(OutError::NotARequestRequisition);
    }

    if requisition_row.status != RequisitionStatus::Draft {
        return Err(OutError::CannotEditRequisition);
    }

    if let (Some(program_id), Some(order_type)) =
        (&requisition_row.program_id, &requisition_row.order_type)
    {
        let (within_limit, max_items) = check_emergency_order_within_max_items_limit(
            connection,
            program_id,
            order_type,
            requisition_lines,
        )
        .map_err(|e| match e {
            OrderTypeNotFoundError::OrderTypeNotFound => OutError::OrderTypeNotFound,
            OrderTypeNotFoundError::DatabaseError(repository_error) => {
                OutError::DatabaseError(repository_error)
            }
        })?;

        if !within_limit {
            return Err(OutError::OrderingTooManyItems(max_items));
        }
    }

    let other_party_id = match &input.other_party_id {
        None => return Ok((requisition_row, status_changed)),
        Some(other_party_id) => other_party_id,
    };

    let other_party = check_other_party(
        connection,
        store_id,
        other_party_id,
        CheckOtherPartyType::Supplier,
    )
    .map_err(|e| match e {
        OtherPartyErrors::OtherPartyDoesNotExist => OutError::OtherPartyDoesNotExist {},
        OtherPartyErrors::OtherPartyNotVisible => OutError::OtherPartyNotVisible,
        OtherPartyErrors::TypeMismatched => OutError::OtherPartyNotASupplier,
        OtherPartyErrors::DatabaseError(repository_error) => {
            OutError::DatabaseError(repository_error)
        }
    })?;

    other_party
        .store_id()
        .ok_or(OutError::OtherPartyIsNotAStore)?;

    Ok((requisition_row, status_changed))
}

use chrono::NaiveDate;

use crate::schema::{
    InvoiceLineRow, InvoiceLineRowType, InvoiceRow, InvoiceRowStatus, InvoiceRowType,
};

use super::MockData;

pub fn mock_test_unallocated_line() -> MockData {
    let mut result = MockData::new();
    result.invoices.push(mock_unallocated_line_invoice());
    result.invoice_lines.push(mock_unallocated_line_line_1());
    result
}

pub fn mock_unallocated_line_invoice() -> InvoiceRow {
    InvoiceRow {
        id: "unallocated_line_invoice".to_owned(),
        name_id: "name_store_a".to_owned(),
        store_id: "store_a".to_owned(),
        invoice_number: 1,
        r#type: InvoiceRowType::OutboundShipment,
        status: InvoiceRowStatus::New,
        on_hold: false,
        comment: None,
        their_reference: None,
        created_datetime: NaiveDate::from_ymd(1970, 1, 5).and_hms_milli(15, 30, 0, 0),
        allocated_datetime: None,
        picked_datetime: None,
        shipped_datetime: None,
        delivered_datetime: None,
        verified_datetime: None,
        color: None,
    }
}

pub fn mock_unallocated_line_line_1() -> InvoiceLineRow {
    InvoiceLineRow {
        id: "mock_unallocated_line_line_1".to_owned(),
        invoice_id: "unallocated_line_invoice".to_owned(),
        item_id: "item_a".to_owned(),
        item_name: "Item A".to_owned(),
        item_code: "item_a_code".to_owned(),
        stock_line_id: None,
        location_id: None,
        batch: None,
        expiry_date: None,
        pack_size: 1,
        cost_price_per_pack: 0.0,
        sell_price_per_pack: 0.0,
        total_before_tax: 0.0,
        total_after_tax: 0.0,
        tax: None,
        r#type: InvoiceLineRowType::UnallocatedStock,
        number_of_packs: 1,
        note: None,
    }
}

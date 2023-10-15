use super::{
    barcode_row::barcode, clinician_row::clinician, invoice_line::invoice_stats,
    invoice_line_row::invoice_line, invoice_row::invoice, item_row::item, location_row::location,
    master_list_line_row::master_list_line, master_list_name_join::master_list_name_join,
    master_list_row::master_list, name_row::name, name_store_join::name_store_join,
    name_tag_join::name_tag_join, period::period,
    program_requisition::program_requisition_order_type_row::program_requisition_order_type,
    program_requisition::program_requisition_settings_row::program_requisition_settings,
    program_row::program, requisition_line_row::requisition_line, requisition_row::requisition,
    sensor_row::sensor, stock_line_row::stock_line, stocktake_line_row::stocktake_line,
    stocktake_row::stocktake, store_row::store,
    temperature_breach_config_row::temperature_breach_config,
    temperature_breach_row::temperature_breach, temperature_log_row::temperature_log,
    unit_row::unit, user_row::user_account,
};

allow_tables_to_appear_in_same_query!(
    unit,
    location,
    sensor,
    temperature_log,
    temperature_breach,
    temperature_breach_config,
    item,
    stock_line,
    name,
    requisition,
    requisition_line,
    store,
    invoice,
    invoice_line,
    invoice_stats,
    user_account,
    name_store_join,
    master_list_line,
    master_list_name_join,
    stocktake,
    stocktake_line,
    master_list,
    program,
    program_requisition_settings,
    program_requisition_order_type,
    period,
    name_tag_join,
    barcode,
    clinician,
);

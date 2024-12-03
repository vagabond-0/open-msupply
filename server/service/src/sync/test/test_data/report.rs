use crate::sync::test::{TestSyncIncomingRecord, TestSyncOutgoingRecord};
use repository::{ContextType, ReportRow, ReportRowDelete, ReportType};
use serde_json::json;

const TABLE_NAME: &str = "report";

// const REPORT_1: (&str, &str) = (
//     "76B6C424E1935C4DAF36A7A8F451FE72",
//     r#"{
//         "ID": "76B6C424E1935C4DAF36A7A8F451FE72",
//         "report_name": "Test",
//         "report_blob": "blob",
//         "permission_ID": "",
//         "last_updated": "0000-00-00",
//         "type": "cus",
//         "user_created_ID": "0763E2E3053D4C478E1E6B6B03FEC207",
//         "Custom_name": "Test",
//         "Comment": "Test comment",
//         "default": false,
//         "context": "Stock Take",
//         "editor": "omsupply",
//         "orientation": "",
//         "disabled": false,
//         "template": "template data",
//         "sub_context": "",
//         "form_schema_ID": ""
//     }"#,
// );

//

const REPORT_1: (&str, &str) = (
    "76B6C424E1935C4DAF36A7A8F451FE72",
    r#"{
        "id": "76B6C424E1935C4DAF36A7A8F451FE72",
        "name": "test name",
        "type": "OmSupply",
        "template:" "template data",
        "context": "report",
        "comment": "Test comment",
        "sub_context": "",
        "argument_schema_id": "",
        "is_custom": false,
        "version": "2.0",
        "code": "test code",
    }"#,
);

fn report_1() -> ReportRow {
    ReportRow {
        id: REPORT_1.0.to_string(),
        name: "test name".to_string(),
        r#type: ReportType::OmSupply,
        template: "template data".to_string(),
        context: ContextType::Report,
        comment: Some("Test comment".to_string()),
        sub_context: Some("".to_string()),
        argument_schema_id: Some("".to_string()),
        is_custom: false,
        version: "2.0".to_string(),
        code: "test code".to_string(),
    }
}

pub(crate) fn test_pull_upsert_records() -> Vec<TestSyncIncomingRecord> {
    vec![TestSyncIncomingRecord::new_pull_upsert(
        TABLE_NAME,
        REPORT_1,
        ReportRow {
            id: REPORT_1.0.to_string(),
            name: "Test".to_string(),
            r#type: ReportType::OmSupply,
            template: "template data".to_string(),
            context: ContextType::Stocktake,
            comment: Some("Test comment".to_string()),
            sub_context: None,
            argument_schema_id: None,
            is_custom: true,
            version: "1.0".to_string(),
            code: REPORT_1.0.to_string(), // for now any thing sync'd from mSupply has the same id as code
        },
    )]
}

pub(crate) fn test_pull_delete_records() -> Vec<TestSyncIncomingRecord> {
    vec![TestSyncIncomingRecord::new_pull_delete(
        TABLE_NAME,
        REPORT_1.0,
        ReportRowDelete(REPORT_1.0.to_string()),
    )]
}

pub(crate) fn test_v6_central_push_records() -> Vec<TestSyncOutgoingRecord> {
    vec![TestSyncOutgoingRecord {
        table_name: TABLE_NAME.to_string(),
        record_id: REPORT_1.0.to_string(),
        push_data: json!(report_1()),
    }]
}

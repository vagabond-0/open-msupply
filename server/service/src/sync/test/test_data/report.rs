use crate::sync::test::{TestSyncIncomingRecord, TestSyncOutgoingRecord};
use repository::{ContextType, ReportRow, ReportRowDelete, ReportType};
use serde_json::json;

const TABLE_NAME: &str = "report";

const REPORT_1: (&str, &str) = (
    "76B6C424E1935C4DAF36A7A8F451FE72",
    r#"{
        "id": "76B6C424E1935C4DAF36A7A8F451FE72",
        "name": "test name",
        "type": "OmSupply",
        "template": "template data",
        "context": "REPORT",
        "is_custom": false,
        "version": "2.0",
        "code": "test code"
    }"#,
);

fn report_1() -> ReportRow {
    ReportRow {
        id: REPORT_1.0.to_string(),
        name: "test name".to_string(),
        r#type: ReportType::OmSupply,
        template: "template data".to_string(),
        context: ContextType::Report,
        comment: None,
        sub_context: None,
        argument_schema_id: None,
        is_custom: false,
        version: "2.0".to_string(),
        code: "test code".to_string(),
    }
}

pub(crate) fn test_pull_upsert_records() -> Vec<TestSyncIncomingRecord> {
    vec![TestSyncIncomingRecord::new_pull_upsert(
        TABLE_NAME,
        REPORT_1,
        report_1(),
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

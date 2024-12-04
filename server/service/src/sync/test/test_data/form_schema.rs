use crate::sync::test::{TestSyncIncomingRecord, TestSyncOutgoingRecord};
use repository::FormSchemaJson;
use serde_json::json;

const TABLE_NAME: &str = "form_schema";

const FORM_SCHEMA: (&str, &str) = (
    "76B6C424E1935C4DAF36A7A8F451FE23",
    r#"{
        "id": "76B6C424E1935C4DAF36A7A8F451FE23",
        "type": "test",
        "json_schema": "test",
        "ui_schema": "test"
    }"#,
);

fn form_schema() -> FormSchemaJson {
    FormSchemaJson {
        id: FORM_SCHEMA.0.to_string(),
        r#type: "test".to_string(),
        json_schema: json!("test"),
        ui_schema: json!("test"),
    }
}

pub(crate) fn test_pull_upsert_records() -> Vec<TestSyncIncomingRecord> {
    vec![TestSyncIncomingRecord::new_pull_upsert(
        TABLE_NAME,
        FORM_SCHEMA,
        form_schema(),
    )]
}

pub(crate) fn test_v6_central_push_records() -> Vec<TestSyncOutgoingRecord> {
    vec![TestSyncOutgoingRecord {
        table_name: TABLE_NAME.to_string(),
        record_id: FORM_SCHEMA.0.to_string(),
        push_data: json!(form_schema()),
    }]
}

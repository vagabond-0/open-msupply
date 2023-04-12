use crate::sync::{
    test::integration::{
        central_server_configurations::NewSiteProperties, SyncRecordTester, TestStepData,
    },
    translations::{IntegrationRecords, PullDeleteRecord, PullDeleteRecordTable, PullUpsertRecord},
};
use repository::{PeriodRow, PeriodScheduleRow};

use chrono::NaiveDate;
use serde_json::json;
use util::uuid::uuid;

pub(crate) struct PeriodScheduleAndPeriodTester;

impl SyncRecordTester for PeriodScheduleAndPeriodTester {
    fn test_step_data(&self, _: &NewSiteProperties) -> Vec<TestStepData> {
        let mut result = Vec::new();

        // STEP 1 - insert
        let period_schedule_1 = PeriodScheduleRow {
            id: uuid(),
            name: "Monthly".to_string(),
        };
        let period_schedule_1_json = json!({
            "ID": period_schedule_1.id,
            "name":  period_schedule_1.name,
        });

        let period_1 = PeriodRow {
            id: uuid(),
            period_schedule_id: period_schedule_1.id.clone(),
            name: "April 2023".to_string(),
            start_date: NaiveDate::from_ymd_opt(2023, 04, 01).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2023, 04, 30).unwrap(),
        };

        let period_1_json = json!({
            "ID": period_1.id,
            "periodScheduleID": period_schedule_1.id,
            "startDate": "2023-04-01",
            "endDate": "2023-04-30",
            "name":  period_1.name,
        });

        let period_schedule_2 = PeriodScheduleRow {
            id: uuid(),
            name: "Yearly".to_string(),
        };

        let period_schedule_2_json = json!({
            "ID": period_schedule_2.id,
            "name":  period_schedule_2.name,
        });

        let period_2 = PeriodRow {
            id: uuid(),
            period_schedule_id: period_schedule_2.id.clone(),
            name: "2023".to_string(),
            start_date: NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
        };

        let period_2_json = json!({
            "ID": period_2.id,
            "periodScheduleID": period_schedule_2.id,
            "startDate": "2023-01-01",
            "endDate": "2023-12-31",
            "name":  period_2.name,
        });

        result.push(TestStepData {
            central_upsert: json!({
                "periodSchedule": [period_schedule_1_json, period_schedule_2_json],
                "period": [period_1_json, period_2_json]
            }),
            central_delete: json!({}),
            integration_records: IntegrationRecords::from_upserts(vec![
                PullUpsertRecord::PeriodSchedule(period_schedule_1.clone()),
                PullUpsertRecord::PeriodSchedule(period_schedule_2),
                PullUpsertRecord::Period(period_1.clone()),
                PullUpsertRecord::Period(period_2),
            ]),
        });

        // STEP 2 - deletes
        result.push(TestStepData {
            central_upsert: json!({}),
            central_delete: json!({ "periodSchedule": [period_schedule_1.id], "period": [period_1.id] }),
            integration_records: IntegrationRecords::from_deletes(vec![
                PullDeleteRecord {
                    id: period_1.id,
                    table: PullDeleteRecordTable::Period,
                },
                PullDeleteRecord {
                    id: period_schedule_1.id,
                    table: PullDeleteRecordTable::PeriodSchedule,
                },
            ]),
        });
        result
    }
}

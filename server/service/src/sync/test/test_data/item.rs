use crate::sync::{
    test::{IntegrationOperation, PullTranslateResult, TestSyncIncomingRecord},
    translations::item::ordered_simple_json,
};
use repository::{
    category_row::CategoryRow, item_category_row::ItemCategoryRow, mock::MockData, ItemRow,
    ItemRowDelete, ItemType, SyncAction, SyncBufferRow,
};

const TABLE_NAME: &str = "item";

const ITEM_1: (&str, &str) = (
    "8F252B5884B74888AAB73A0D42C09E7A",
    r#"{
    "ID": "8F252B5884B74888AAB73A0D42C09E7A",
    "item_name": "Non stock items",
    "start_of_year_date": "0000-00-00",
    "manufacture_method": "",
    "default_pack_size": 1,
    "dose_picture": "",
    "atc_category": "",
    "medication_purpose": "",
    "instructions": "",
    "user_field_7": false,
    "flags": "",
    "ddd_value": "",
    "code": "NSI",
    "other_names": "",
    "type_of": "non_stock",
    "price_editable": false,
    "margin": 0,
    "barcode_spare": "",
    "spare_ignore_for_orders": false,
    "sms_pack_size": 0,
    "expiry_date_mandatory": false,
    "volume_per_pack": 0,
    "department_ID": "",
    "weight": 0,
    "essential_drug_list": false,
    "catalogue_code": "",
    "indic_price": 0,
    "user_field_1": "",
    "spare_hold_for_issue": false,
    "builds_only": false,
    "reference_bom_quantity": 0,
    "use_bill_of_materials": false,
    "description": "",
    "spare_hold_for_receive": false,
    "Message": "",
    "interaction_group_ID": "",
    "spare_pack_to_one_on_receive": false,
    "cross_ref_item_ID": "",
    "strength": "",
    "user_field_4": false,
    "user_field_6": "",
    "spare_internal_analysis": 0,
    "user_field_2": "",
    "user_field_3": "",
    "ddd factor": 0,
    "account_stock_ID": "52923505A91447B9923BA34A4F332014",
    "account_purchases_ID": "330ACC81721C4126BD5DD6769466C5C4",
    "account_income_ID": "EF34ADD07C014AB8914E30CA2E3FEA8D",
    "unit_ID": "",
    "outer_pack_size": 0,
    "category_ID": "",
    "ABC_category": "",
    "warning_quantity": 0,
    "user_field_5": 0,
    "print_units_in_dis_labels": false,
    "volume_per_outer_pack": 0,
    "normal_stock": false,
    "critical_stock": false,
    "spare_non_stock": false,
    "non_stock_name_ID": "",
    "is_sync": false,
    "sms_code": "",
    "category2_ID": "",
    "category3_ID": "",
    "buy_price": 0,
    "VEN_category": "",
    "universalcodes_code": "",
    "universalcodes_name": "",
    "kit_data": null,
    "custom_data": null,
    "doses": 0,
    "is_vaccine": false,
    "restricted_location_type_ID": "",
    "product_specifications": ""
}"#,
);

const ITEM_2: (&str, &str) = (
    "8F252B5884B74888AAB73A0D42C09E7F",
    r#"{
    "ID": "8F252B5884B74888AAB73A0D42C09E7F",
    "item_name": "Non stock items 2",
    "start_of_year_date": "0000-00-00",
    "manufacture_method": "",
    "default_pack_size": 2,
    "dose_picture": "",
    "atc_category": "",
    "medication_purpose": "",
    "instructions": "",
    "user_field_7": false,
    "flags": "",
    "ddd_value": "",
    "code": "NSI",
    "other_names": "",
    "type_of": "general",
    "price_editable": false,
    "margin": 0,
    "barcode_spare": "",
    "spare_ignore_for_orders": false,
    "sms_pack_size": 0,
    "expiry_date_mandatory": false,
    "volume_per_pack": 0,
    "department_ID": "",
    "weight": 0,
    "essential_drug_list": false,
    "catalogue_code": "",
    "indic_price": 0,
    "user_field_1": "",
    "spare_hold_for_issue": false,
    "builds_only": false,
    "reference_bom_quantity": 0,
    "use_bill_of_materials": false,
    "description": "",
    "spare_hold_for_receive": false,
    "Message": "",
    "interaction_group_ID": "",
    "spare_pack_to_one_on_receive": false,
    "cross_ref_item_ID": "",
    "strength": "",
    "user_field_4": false,
    "user_field_6": "",
    "spare_internal_analysis": 0,
    "user_field_2": "",
    "user_field_3": "",
    "ddd factor": 0,
    "account_stock_ID": "52923505A91447B9923BA34A4F332014",
    "account_purchases_ID": "330ACC81721C4126BD5DD6769466C5C4",
    "account_income_ID": "EF34ADD07C014AB8914E30CA2E3FEA8D",
    "unit_ID": "A02C91EB6C77400BA783C4CD7C565F29",
    "outer_pack_size": 0,
    "category_ID": "",
    "ABC_category": "",
    "warning_quantity": 0,
    "user_field_5": 0,
    "print_units_in_dis_labels": false,
    "volume_per_outer_pack": 0,
    "normal_stock": false,
    "critical_stock": false,
    "spare_non_stock": false,
    "non_stock_name_ID": "",
    "is_sync": false,
    "sms_code": "",
    "category2_ID": "",
    "category3_ID": "",
    "buy_price": 0,
    "VEN_category": "",
    "universalcodes_code": "",
    "universalcodes_name": "",
    "kit_data": null,
    "custom_data": null,
    "doses": 0,
    "is_vaccine": false,
    "restricted_location_type_ID": "",
    "product_specifications": ""
}"#,
);

const ITEM_3_VACCINE: (&str, &str) = (
    "F078B01C94DF4A5BA1EC0408CDD46B55",
    r#"{
    "ABC_category": "",
    "ID": "F078B01C94DF4A5BA1EC0408CDD46B55",
    "Message": "",
    "VEN_category": "",
    "account_income_ID": "EF34ADD07C014AB8914E30CA2E3FEA8D",
    "account_purchases_ID": "330ACC81721C4126BD5DD6769466C5C4",
    "account_stock_ID": "52923505A91447B9923BA34A4F332014",
    "atc_category": "",
    "barcode_spare": "",
    "builds_only": false,
    "buy_price": 0,
    "catalogue_code": "",
    "category2_ID": "",
    "category3_ID": "",
    "category_ID": "FA6FC67251CC4560AC7FED0C0B23E5A0",
    "code": "Covid19-Pfizer-BioNTech",
    "critical_stock": false,
    "cross_ref_item_ID": "",
    "custom_data": null,
    "ddd factor": 0,
    "ddd_value": "",
    "default_pack_size": 1,
    "department_ID": "",
    "description": "",
    "dose_picture": "",
    "doses": 1,
    "essential_drug_list": false,
    "expiry_date_mandatory": false,
    "flags": "",
    "indic_price": 0,
    "instructions": "",
    "interaction_group_ID": "",
    "is_sync": false,
    "is_vaccine": true,
    "item_name": "Covid-19 Vaccine",
    "kit_data": null,
    "manufacture_method": "",
    "margin": 0,
    "medication_purpose": "",
    "non_stock_name_ID": "",
    "normal_stock": true,
    "other_names": "",
    "outer_pack_size": 0,
    "price_editable": false,
    "print_units_in_dis_labels": false,
    "product_specifications": "",
    "reference_bom_quantity": 0,
    "restricted_location_type_ID": "",
    "sms_code": "",
    "sms_pack_size": 0,
    "spare_hold_for_issue": false,
    "spare_hold_for_receive": false,
    "spare_ignore_for_orders": false,
    "spare_internal_analysis": 0,
    "spare_non_stock": false,
    "spare_pack_to_one_on_receive": false,
    "start_of_year_date": "0000-00-00",
    "strength": "",
    "type_of": "general",
    "unit_ID": "97674EFD5DFD4D8CABCAF58AAB4ED054",
    "universalcodes_code": "3fd9b240c",
    "universalcodes_name": "",
    "use_bill_of_materials": false,
    "user_field_1": "",
    "user_field_2": "",
    "user_field_3": "",
    "user_field_4": false,
    "user_field_5": 0,
    "user_field_6": "",
    "user_field_7": false,
    "volume_per_outer_pack": 0,
    "volume_per_pack": 0,
    "warning_quantity": 0,
    "weight": 0
}"#,
);

pub(crate) fn test_pull_upsert_records() -> Vec<TestSyncIncomingRecord> {
    vec![
        TestSyncIncomingRecord::new_pull_upsert(
            TABLE_NAME,
            (ITEM_1.0, &ordered_simple_json(ITEM_1.1).unwrap()),
            ItemRow {
                id: ITEM_1.0.to_owned(),
                name: "Non stock items".to_owned(),
                code: "NSI".to_owned(),
                unit_id: None,
                r#type: ItemType::NonStock,
                legacy_record: ordered_simple_json(ITEM_1.1).unwrap(),
                default_pack_size: 1.0,
                is_active: true,
                is_vaccine: false,
                vaccine_doses: 0,
                ..Default::default()
            },
        ),
        TestSyncIncomingRecord::new_pull_upsert(
            TABLE_NAME,
            (ITEM_2.0, &ordered_simple_json(ITEM_2.1).unwrap()),
            ItemRow {
                id: ITEM_2.0.to_owned(),
                name: "Non stock items 2".to_owned(),
                code: "NSI".to_owned(),
                unit_id: Some("A02C91EB6C77400BA783C4CD7C565F29".to_owned()),
                r#type: ItemType::Stock,
                legacy_record: ordered_simple_json(ITEM_2.1).unwrap(),
                default_pack_size: 2.0,
                is_active: true,
                is_vaccine: false,
                vaccine_doses: 0,
                ..Default::default()
            },
        ),
        TestSyncIncomingRecord {
            translated_record: PullTranslateResult::IntegrationOperations(vec![
                IntegrationOperation::upsert(ItemRow {
                    id: ITEM_3_VACCINE.0.to_owned(),
                    name: "Covid-19 Vaccine".to_owned(),
                    code: "Covid19-Pfizer-BioNTech".to_owned(),
                    unit_id: Some("97674EFD5DFD4D8CABCAF58AAB4ED054".to_owned()),
                    r#type: ItemType::Stock,
                    legacy_record: ordered_simple_json(ITEM_3_VACCINE.1).unwrap(),
                    default_pack_size: 1.0,
                    is_active: true,
                    is_vaccine: true,
                    vaccine_doses: 1,
                    ..Default::default()
                }),
                IntegrationOperation::upsert(ItemCategoryRow {
                    id: format!(
                        "{}-{}",
                        ITEM_3_VACCINE.0, "FA6FC67251CC4560AC7FED0C0B23E5A0"
                    ),
                    item_id: ITEM_3_VACCINE.0.to_owned(),
                    category_id: "FA6FC67251CC4560AC7FED0C0B23E5A0".to_owned(),
                    deleted_datetime: None,
                }),
            ]),
            sync_buffer_row: SyncBufferRow {
                table_name: TABLE_NAME.to_string(),
                record_id: ITEM_3_VACCINE.0.to_owned(),
                data: ITEM_3_VACCINE.1.to_owned(),
                action: SyncAction::Upsert,
                ..Default::default()
            },
            extra_data: Some(MockData {
                categories: vec![CategoryRow {
                    id: "FA6FC67251CC4560AC7FED0C0B23E5A0".to_owned(),
                    ..Default::default()
                }],
                ..Default::default()
            }),
        },
    ]
}

pub(crate) fn test_pull_delete_records() -> Vec<TestSyncIncomingRecord> {
    vec![TestSyncIncomingRecord::new_pull_delete(
        TABLE_NAME,
        ITEM_1.0,
        ItemRowDelete(ITEM_1.0.to_string()),
    )]
}

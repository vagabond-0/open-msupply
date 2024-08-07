use super::{version::Version, Migration};

mod add_new_table;
mod delete_after_rc_drop_table;

pub(crate) struct V2_03_00;

impl Migration for V2_03_00 {
    fn version(&self) -> Version {
        Version::from_str("2.3.0")
    }

    fn migrate_fragments(&self) -> Vec<Box<dyn super::MigrationFragment>> {
        vec![
            Box::new(delete_after_rc_drop_table::Migrate),
            Box::new(add_new_table::Migrate),
        ]
    }
}

#[cfg(test)]
#[actix_rt::test]
async fn migration_2_03_00() {
    use v2_02_00::V2_02_00;

    use crate::migrations::*;
    use crate::test_db::*;

    let previous_version = V2_02_00.version();
    let version = V2_03_00.version();

    let SetupResult { connection, .. } = setup_test(SetupOption {
        db_name: &format!("migration_{version}"),
        version: Some(previous_version.clone()),
        ..Default::default()
    })
    .await;

    // Run this migration
    migrate(&connection, Some(version.clone())).unwrap();
    assert_eq!(get_database_version(&connection), version);
}

use crate::migrations::*;

pub(crate) struct Migrate;

impl MigrationFragment for Migrate {
    fn identifier(&self) -> &'static str {
        "add_new_table_with_new_id"
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        sql!(
            connection,
            r#"
                CREATE TABLE new_table (
                    id TEXT NOT NULL PRIMARY KEY,
                    data {DOUBLE} NOT NULL,
                    extra_data TEXT
                )
            "#
        )?;

        Ok(())
    }
}

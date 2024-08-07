use crate::migrations::*;

pub(crate) struct Migrate;

impl MigrationFragment for Migrate {
    fn identifier(&self) -> &'static str {
        // Identifier should be different
        "add_new_table_should_be_different"
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        sql!(
            connection,
            r#"
                CREATE TABLE new_table (
                    id TEXT NOT NULL PRIMARY KEY,
                    data NOT NULL {DOUBLE},
                    extra_data TEXT
                )
            "#
        )?;

        Ok(())
    }
}

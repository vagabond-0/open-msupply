use crate::migrations::*;

pub(crate) struct Migrate;

impl MigrationFragment for Migrate {
    fn identifier(&self) -> &'static str {
        "add_new_table"
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        sql!(
            connection,
            r#"
                CREATE TABLE new_table (
                    id TEXT NOT NULL PRIMARY KEY,
                    data TEXT
                )
            "#
        )?;

        Ok(())
    }
}

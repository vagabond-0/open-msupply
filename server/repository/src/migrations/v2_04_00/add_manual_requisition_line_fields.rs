use crate::migrations::*;

pub(crate) struct Migrate;

impl MigrationFragment for Migrate {
    fn identifier(&self) -> &'static str {
        "add_manual_requisition_line_fields"
    }

    fn migrate(&self, connection: &StorageConnection) -> anyhow::Result<()> {
        if cfg!(feature = "postgres") {
            sql!(
                connection,
                r#"
                ALTER TABLE requisition_line ADD initial_stock_on_hand_units {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD incoming_units {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD outgoing_units {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD loss_in_units {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD addition_in_units {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD available_units {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD expiring_units {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD days_out_of_stock {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD months_of_stock {DOUBLE} NOT NULL DEFAULT 0;
                ALTER TABLE requisition_line ADD option_ID REFERENCES return_reason(id);
                "#
            )?;
        }

        Ok(())
    }
}

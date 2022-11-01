CREATE TABLE user_permission (
    id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES user_account(id),
    store_id TEXT NOT NULL REFERENCES store(id),
    permission TEXT CHECK (permission IN (
        'STORE_ACCESS',
        'LOCATION_MUTATE',
        'STOCK_LINE_QUERY',
        'STOCKTAKE_QUERY',
        'STOCKTAKE_MUTATE',
        'REQUISITION_QUERY',
        'REQUISITION_MUTATE',
        'OUTBOUND_SHIPMENT_QUERY',
        'OUTBOUND_SHIPMENT_MUTATE',
        'INBOUND_SHIPMENT_QUERY',
        'INBOUND_SHIPMENT_MUTATE',
        'REPORT',
        'LOG_QUERY',
        'SERVER_ADMIN',
        'DOCUMENT',
        'PATIENT_QUERY',
        'PATIENT_MUTATE',
        'DOCUMENT_ENCOUNTER_QUERY',
        'DOCUMENT_ENCOUNTER_MUTATE',
        'DOCUMENT_PROGRAM_QUERY',
        'DOCUMENT_PROGRAM_MUTATE'
    )) NOT NULL,
    context TEXT
)

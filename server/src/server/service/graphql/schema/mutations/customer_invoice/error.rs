use async_graphql::Object;

use crate::server::service::graphql::schema::types::NameNode;

pub struct CannotChangeStatusBackToDraftError;

#[Object]
impl CannotChangeStatusBackToDraftError {
    pub async fn description(&self) -> &'static str {
        "Once confirmed or finalised, an invoice cannot be changed back to a draft."
    }
}

pub struct CanOnlyEditInvoicesInLoggedInStoreError;

#[Object]
impl CanOnlyEditInvoicesInLoggedInStoreError {
    pub async fn description(&self) -> &'static str {
        "Once finalised, an invoice cannot be edited."
    }
}

pub struct FinalisedInvoiceIsNotEditableError;

#[Object]
impl FinalisedInvoiceIsNotEditableError {
    pub async fn description(&self) -> &'static str {
        "Once finalised, an invoice cannot be edited."
    }
}

pub struct InvoiceDoesNotBelongToCurrentStoreError(pub String);

#[Object]
impl InvoiceDoesNotBelongToCurrentStoreError {
    pub async fn description(&self) -> String {
        format!(
            "Invoice with id '{}' does not belong to the current store.",
            self.0
        )
    }
}

pub struct InvoiceNotFoundError();

#[Object]
impl InvoiceNotFoundError {
    pub async fn description(&self) -> &'static str {
        "Invoice not found."
    }
}

pub struct OtherPartyCannotBeThisStoreError;

#[Object]
impl OtherPartyCannotBeThisStoreError {
    pub async fn description(&self) -> &'static str {
        "Other party must be another store."
    }
}

pub struct OtherPartyIdMissingError;

#[Object]
impl OtherPartyIdMissingError {
    pub async fn description(&self) -> &'static str {
        "Other party id missing."
    }
}

pub struct OtherPartyIdNotFoundError;

#[Object]
impl OtherPartyIdNotFoundError {
    pub async fn description(&self) -> &'static str {
        "Other party not found."
    }
}

pub struct OtherPartyNotACustomerError(pub NameNode);

#[Object]
impl OtherPartyNotACustomerError {
    pub async fn description(&self) -> &'static str {
        "Other party name is not a customer"
    }

    pub async fn other_party(&self) -> &NameNode {
        &self.0
    }
}

pub struct NotACustomerInvoiceError;

#[Object]
impl NotACustomerInvoiceError {
    pub async fn description(&self) -> &'static str {
        "Not a customer invoice."
    }
}

pub struct InternalError(pub String);

#[Object]
impl InternalError {
    pub async fn description(&self) -> &'static str {
        "Internal Error"
    }

    pub async fn msg(&self) -> &String {
        &self.0
    }
}

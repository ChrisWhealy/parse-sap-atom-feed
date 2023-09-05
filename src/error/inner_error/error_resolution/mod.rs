use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an internal SAP OData `<innererror>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResolution {
    #[serde(rename = "SAP_Transaction")]
    pub sap_transaction: String,
    #[serde(rename = "SAP_Note")]
    pub sap_note: String,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

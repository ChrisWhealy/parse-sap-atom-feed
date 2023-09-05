pub mod application;
pub mod error_details;
pub mod error_resolution;

use application::Application;
use error_details::ErrorDetails;
use error_resolution::ErrorResolution;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an internal SAP OData `<innererror>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct InnerError {
    pub application: Application,

    #[serde(rename = "transactionid")]
    pub transaction_id: String,

    pub timestamp: String,

    #[serde(rename = "Error_Resolution")]
    pub error_resolution: ErrorResolution,

    #[serde(rename = "errordetails")]
    pub error_details: ErrorDetails,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

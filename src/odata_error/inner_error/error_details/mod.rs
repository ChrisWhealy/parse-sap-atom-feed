use crate::{deserializers::edm_string::to_bool, xml::default_false};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an internal SAP OData `<errordetails>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(rename = "errordetail")]
    pub error_detail: Option<Vec<ErrorDetail>>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an internal SAP OData `<errordetail>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "ContentID")]
    pub content_id: Option<String>,
    pub code: String,
    pub message: String,

    #[serde(rename = "propertyref")]
    pub property_ref: Option<String>,
    pub severity: String,
    pub target: Option<String>,

    #[serde(deserialize_with = "to_bool", default = "default_false")]
    pub transition: bool,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

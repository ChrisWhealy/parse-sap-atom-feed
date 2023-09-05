pub mod inner_error;

use crate::xml::{default_xml_language, default_xml_namespace};
use inner_error::InnerError;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an internal SAP OData `<error xmlns="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata">` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct ODataError {
    #[serde(rename = "xmlns", default = "default_xml_namespace")]
    pub xml_namespace: String,

    pub code: String,

    #[serde(rename = "xml:lang", default = "default_xml_language")]
    pub message_language: String,
    pub message: String,

    #[serde(rename = "innererror")]
    pub inner_error: InnerError,
}

impl std::str::FromStr for ODataError {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

use crate::{deserializers::default_false, xml::default_xml_data_services_scheme};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<category>` tag within an Atom `<entry>`
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCategory {
    #[serde(rename = "@term")]
    pub term: String,
    #[serde(rename = "@scheme", default = "default_xml_data_services_scheme")]
    pub scheme: String,
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@fixed", default = "default_false")]
    pub fixed: bool,
}

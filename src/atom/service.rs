use super::{link::AtomLink, workspace::AtomWorkspace};
use crate::xml::{
    default_xml_language, default_xml_namespace_app, default_xml_namespace_atom,
    default_xml_namespace_m, default_xml_namespace_sap,
};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom service document `<app:service>`
///
/// ***WARNING:***<br>`quick-xml` strips namespace identifiers from XML tag names, and from certain attribute names!
///
/// Tag names such as `<app:service>` and `<atom:title>` will appear simply as `<service>` and `<title>`.
///
/// Attribute names prefixed with `xml` such as `xml:lang` or `xml:base` will be modified to `lang` and `base`, but
/// `xmlns:app` or `xmlns:atom` will appear without modification
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomService {
    #[serde(rename = "@xmlns:app", default = "default_xml_namespace_app")]
    pub namespace_app: String,
    #[serde(rename = "@xmlns:atom", default = "default_xml_namespace_atom")]
    pub namespace_atom: Option<String>,
    #[serde(rename = "@xmlns:m", default = "default_xml_namespace_m")]
    pub namespace_m: String,
    #[serde(rename = "@xmlns:sap", default = "default_xml_namespace_sap")]
    pub namespace_sap: String,
    #[serde(rename = "@lang", default = "default_xml_language")]
    pub language: String,
    #[serde(rename = "@base")]
    pub base_url: String,
    pub workspace: AtomWorkspace,
    #[serde(rename = "link")]
    pub links: Vec<AtomLink>,
}

impl std::str::FromStr for AtomService {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

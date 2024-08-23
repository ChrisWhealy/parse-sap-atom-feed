use crate::xml::default_xml_namespace_atom;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:link>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomLink {
    // Appears in the XML as the `link` attribute `xmlns:atom`
    #[serde(rename = "@atom", default = "default_xml_namespace_atom")]
    pub xml_namespace_atom: Option<String>,

    #[serde(rename = "@type")]
    pub mime_type: Option<String>,

    #[serde(rename = "@rel")]
    pub rel: String,

    #[serde(rename = "@href")]
    pub href: String,

    #[serde(rename = "@title")]
    pub title: Option<String>,
}

impl std::str::FromStr for AtomLink {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

use crate::xml::default_xml_namespace_atom;
use serde::{Deserialize, Serialize};

pub mod feed;

// use feed::Feed;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:link>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomLink {
    #[serde(rename = "xmlns:atom", default = "default_xml_namespace_atom")]
    pub xml_namespace_atom: Option<String>,

    #[serde(rename = "type")]
    pub mime_type: Option<String>,

    pub rel: String,
    pub href: String,
    pub title: Option<String>,
    // #[serde(rename = "m:inline")]
    // pub inline: Option<Feed<T>>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

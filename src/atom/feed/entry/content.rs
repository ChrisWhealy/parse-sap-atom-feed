use crate::xml::{default_xml_namespace_d, default_xml_namespace_m};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<content>` tag
///
/// # Child Nodes
/// `1:1 m:properties: <T>` where `<T>` is the entity type of this particular entity set
#[derive(Debug, Serialize, Deserialize)]
pub struct Content<T> {
    #[serde(rename = "@type")]
    pub content_type: Option<String>,

    #[serde(rename = "@m", default = "default_xml_namespace_m")]
    pub namespace_m: String,

    #[serde(rename = "@d", default = "default_xml_namespace_d")]
    pub namespace_d: String,

    // If the `src` attribute is populated, then the `properties` element exists as a sibling of <content>
    // If the `src` attribute is missing, then the `properties` element exists as a child within <content>
    #[serde(rename = "@src")]
    pub src: Option<String>,

    // Will only be populated if the above `src` attribute is missing
    pub properties: Option<T>,
}

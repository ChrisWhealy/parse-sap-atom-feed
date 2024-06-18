use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    atom::feed::{AtomLink, entry::content::Content},
    deserializers::{default_false, default_xml_dataservices_scheme},
};

pub mod category;
pub mod content;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<category>` tag with an Atom `<entry>`
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCategory {
    #[serde(rename = "@term")]
    pub term: String,
    #[serde(rename = "@scheme", default = "default_xml_dataservices_scheme")]
    pub scheme: String,
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@fixed", default = "default_false")]
    pub fixed: bool,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<entry>` of type `<T>` where `<T>` is the entity type of this particular entity set
///
/// # Child Nodes
/// `1:1 id`<br>
/// `1:1 title`<br>
/// `1:1 updated`<br>
/// `1:1 category`<br>
/// `1:n link`<br>
/// `1:1 content`<br>
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry<T> {
    // Appears in the XML as the `entry` attribute `m:etag`
    #[serde(rename = "@etag")]
    pub etag: Option<String>,

    pub id: String,
    pub title: String,
    pub updated: String,
    pub category: EntryCategory,

    #[serde(rename = "link")]
    pub links: Vec<AtomLink>,

    pub content: Content<T>,
    pub properties: Option<T>,
}

impl<T> std::str::FromStr for Entry<T>
where
    T: DeserializeOwned,
{
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

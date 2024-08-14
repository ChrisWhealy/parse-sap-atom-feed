use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    atom::feed::{entry::content::Content, AtomLink},
    deserializers::default_false,
    xml::default_xml_data_services_scheme,
};

pub mod category;
pub mod content;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents a `<category>` tag with an Atom `<entry>`
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

    // This `properties` field will only be populated if the `src` attribute of above `<content>` element exists.
    // If the `src` attribute is missing, then this `properties` field will be `None` and there will be a `properties`
    // child within `<content>`
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

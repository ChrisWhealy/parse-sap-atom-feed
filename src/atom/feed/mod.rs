pub mod author;
pub mod entry;

use crate::{
    atom::AtomLink,
    xml::{default_xml_namespace_atom, default_xml_namespace_d, default_xml_namespace_m},
};
use author::Author;
use entry::Entry;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<feed>`
///
/// # Child Nodes
/// `1:1 author`<br>
/// `1:1 link`<br>
/// `0:n entry`
///
/// ***WARNING:***<br>`quick-xml` strips the namespace from both XML tag and attribute names!
#[derive(Debug, Serialize, Deserialize)]
pub struct Feed<T> {
    #[serde(rename = "@xmlns", default = "default_xml_namespace_atom")]
    pub namespace: Option<String>,

    // Appears in the XML as the `feed` attribute `xmlns:m`
    #[serde(rename = "@m", default = "default_xml_namespace_m")]
    pub namespace_m: String,

    // Appears in the XML as the `feed` attribute `xmlns:d`
    #[serde(rename = "@d", default = "default_xml_namespace_d")]
    pub namespace_d: String,

    // Appears in the XML as the `feed` attribute `xmlns:base`
    #[serde(rename = "@base")]
    pub xml_base: Option<String>,

    pub id: String,
    pub title: String,
    pub updated: String,
    pub author: Author,

    #[serde(rename = "link")]
    pub links: Vec<AtomLink>,

    #[serde(rename = "entry")]
    pub entries: Option<Vec<Entry<T>>>,
}

impl<T> std::str::FromStr for Feed<T>
where
    T: DeserializeOwned,
{
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

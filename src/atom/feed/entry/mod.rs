pub mod category;
pub mod content;

use crate::atom::feed::AtomLink;
use content::Content;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
    pub category: String,

    #[serde(rename = "link")]
    pub links: Vec<AtomLink>,

    pub content: Content<T>,
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

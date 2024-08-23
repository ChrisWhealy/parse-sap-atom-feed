use super::collection::AtomCollection;
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:workspace>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomWorkspace {
    pub title: String,

    #[serde(rename = "collection")]
    pub collections: Vec<AtomCollection>,
}

impl std::str::FromStr for AtomWorkspace {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

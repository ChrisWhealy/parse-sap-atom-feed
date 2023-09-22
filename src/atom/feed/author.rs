use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<author>`
#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    #[serde(rename = "@Name")]
    pub name: Option<String>,

    #[serde(rename = "@Uri")]
    pub uri: Option<String>,

    #[serde(rename = "@Email")]
    pub email: Option<String>,
}

use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom `<category>`
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "@term")]
    pub term: String,

    #[serde(rename = "@scheme")]
    pub scheme: String,
}

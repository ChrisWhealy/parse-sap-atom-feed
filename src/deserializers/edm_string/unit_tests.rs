use super::*;
use crate::test_utils::*;

use serde::Deserialize;
use std::str::FromStr;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tests for String deserialization
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct BooleanStringElement {
    #[serde(rename = "@Nullable", deserialize_with = "to_bool")]
    nullable: bool,
    #[serde(rename = "@Sortable", deserialize_with = "to_bool")]
    sortable: bool,
}

impl FromStr for BooleanStringElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_boolean_string() -> Result<(), String> {
    let boolean_string_xml = "<Test Nullable=\"true\" Sortable=\"false\"></Test>";

    match BooleanStringElement::from_str(boolean_string_xml) {
        Ok(result) => {
            handle_test_comparison(&true, &result.nullable)?;
            handle_test_comparison(&false, &result.sortable)
        }
        Err(err) => Err(format!("{err}")),
    }
}

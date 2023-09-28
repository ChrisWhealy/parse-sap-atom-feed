use super::{de_date_to_naive_date_time, de_date_to_optional_naive_date_time};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DateElement {
    #[serde(deserialize_with = "de_date_to_naive_date_time")]
    created_at: chrono::NaiveDateTime,
}

impl std::str::FromStr for DateElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDateElement {
    #[serde(deserialize_with = "de_date_to_optional_naive_date_time")]
    created_at: Option<chrono::NaiveDateTime>,
}

impl std::str::FromStr for OptionalDateElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_naive_date_time() {
    let test_date_time = "2023-08-31T07:11:03.1182110";
    let created_at = chrono::NaiveDateTime::from_str(test_date_time).unwrap();
    let created_at_xml = &format!("<Test><d:CreatedAt>{}</d:CreatedAt></Test>", test_date_time);

    match DateElement::from_str(created_at_xml) {
        Ok(result) => assert_eq!(created_at, result.created_at),
        Err(err) => assert!(false, "{}", err),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_optional_naive_date_time() {
    let test_date_time = "2023-08-31T07:11:03.1182110";
    let created_at = chrono::NaiveDateTime::from_str(test_date_time).unwrap();
    let created_at_xml = &format!("<Test><d:CreatedAt>{}</d:CreatedAt></Test>", test_date_time);

    match OptionalDateElement::from_str(created_at_xml) {
        Ok(result) => assert_eq!(result.created_at, Some(created_at)),
        Err(err) => assert!(false, "{}", err),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_empty_optional_naive_date_time() {
    let created_at_xml = "<Test><d:CreatedAt m:null=\"true\" /></Test>";

    match OptionalDateElement::from_str(created_at_xml) {
        Ok(result) => assert_eq!(result.created_at, None),
        Err(err) => assert!(false, "{}", err),
    }
}

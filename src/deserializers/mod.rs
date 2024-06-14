use chrono::NaiveDateTime;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, str::FromStr};

static ONE: &str = "1";

pub fn default_sap_content_version() -> String {
    String::from(ONE)
}
pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize String to Boolean
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn de_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    bool::from_str(&s).or_else(|_| Ok(false))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize Edm.DateTime to chrono::NaiveDateTime
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[allow(dead_code)]
struct NaiveDateTimeVisitor;

impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
    type Value = chrono::NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an XML tag containing something like either <d:DeliveryDate>2018-01-07T23:00:00.0000000</d:DeliveryDate> or <d:DateOfBirth m:null=\"true\"/>")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match NaiveDateTime::from_str(&value) {
            Ok(ndt) => Ok(ndt),
            Err(err) => Err(de::Error::custom(err.to_string())),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn de_date_to_optional_naive_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    match chrono::NaiveDateTime::deserialize(deserializer) {
        Ok(ndt) => Ok(Some(ndt)),
        Err(_err) => Ok(None),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn de_date_to_naive_date_time<'de, D>(
    deserializer: D,
) -> Result<chrono::NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    chrono::NaiveDateTime::deserialize(deserializer)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

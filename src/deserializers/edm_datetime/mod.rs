use chrono::NaiveDateTime;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, str::FromStr};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Deserialize Edm.DateTime to chrono::NaiveDateTime
#[allow(dead_code)]
struct NaiveDateTimeVisitor;

impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an XML tag a date time formatted either as <d:DeliveryDate>2018-01-07T23:00:00.0000000</d:DeliveryDate> or <d:DateOfBirth m:null=\"true\"/>")
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
pub fn to_naive_date_time_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(NaiveDateTime::deserialize(deserializer).ok())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn to_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    NaiveDateTime::deserialize(deserializer)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

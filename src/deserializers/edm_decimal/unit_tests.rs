use super::*;
use crate::test_utils::*;

use serde::Deserialize;
use std::str::FromStr;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tests for Decimal deserialization
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement00 {
    #[serde(deserialize_with = "to_rust_decimal_0dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement01 {
    #[serde(deserialize_with = "to_rust_decimal_1dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement02 {
    #[serde(deserialize_with = "to_rust_decimal_2dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement03 {
    #[serde(deserialize_with = "to_rust_decimal_3dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement04 {
    #[serde(deserialize_with = "to_rust_decimal_4dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement05 {
    #[serde(deserialize_with = "to_rust_decimal_5dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement06 {
    #[serde(deserialize_with = "to_rust_decimal_6dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement07 {
    #[serde(deserialize_with = "to_rust_decimal_7dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement08 {
    #[serde(deserialize_with = "to_rust_decimal_8dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement09 {
    #[serde(deserialize_with = "to_rust_decimal_9dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement10 {
    #[serde(deserialize_with = "to_rust_decimal_10dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement11 {
    #[serde(deserialize_with = "to_rust_decimal_11dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement12 {
    #[serde(deserialize_with = "to_rust_decimal_12dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement13 {
    #[serde(deserialize_with = "to_rust_decimal_13dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement14 {
    #[serde(deserialize_with = "to_rust_decimal_14dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement15 {
    #[serde(deserialize_with = "to_rust_decimal_15dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement16 {
    #[serde(deserialize_with = "to_rust_decimal_16dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement17 {
    #[serde(deserialize_with = "to_rust_decimal_17dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement18 {
    #[serde(deserialize_with = "to_rust_decimal_18dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement19 {
    #[serde(deserialize_with = "to_rust_decimal_19dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement20 {
    #[serde(deserialize_with = "to_rust_decimal_20dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement21 {
    #[serde(deserialize_with = "to_rust_decimal_21dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement22 {
    #[serde(deserialize_with = "to_rust_decimal_22dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement23 {
    #[serde(deserialize_with = "to_rust_decimal_23dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement24 {
    #[serde(deserialize_with = "to_rust_decimal_24dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement25 {
    #[serde(deserialize_with = "to_rust_decimal_25dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement26 {
    #[serde(deserialize_with = "to_rust_decimal_26dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement27 {
    #[serde(deserialize_with = "to_rust_decimal_27dp")]
    num: Decimal,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement28 {
    #[serde(deserialize_with = "to_rust_decimal_28dp")]
    num: Decimal,
}

impl FromStr for DecimalElement00 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement01 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement02 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement03 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement04 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement05 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement06 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement07 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement08 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement09 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement10 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement11 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement12 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement13 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement14 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement15 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement16 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement17 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement18 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement19 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement20 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement21 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement22 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement23 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement24 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement25 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement26 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement27 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for DecimalElement28 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement00 {
    #[serde(deserialize_with = "to_rust_decimal_0dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement01 {
    #[serde(deserialize_with = "to_rust_decimal_1dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement02 {
    #[serde(deserialize_with = "to_rust_decimal_2dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement03 {
    #[serde(deserialize_with = "to_rust_decimal_3dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement04 {
    #[serde(deserialize_with = "to_rust_decimal_4dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement05 {
    #[serde(deserialize_with = "to_rust_decimal_5dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement06 {
    #[serde(deserialize_with = "to_rust_decimal_6dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement07 {
    #[serde(deserialize_with = "to_rust_decimal_7dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement08 {
    #[serde(deserialize_with = "to_rust_decimal_8dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement09 {
    #[serde(deserialize_with = "to_rust_decimal_9dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement10 {
    #[serde(deserialize_with = "to_rust_decimal_10dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement11 {
    #[serde(deserialize_with = "to_rust_decimal_11dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement12 {
    #[serde(deserialize_with = "to_rust_decimal_12dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement13 {
    #[serde(deserialize_with = "to_rust_decimal_13dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement14 {
    #[serde(deserialize_with = "to_rust_decimal_14dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement15 {
    #[serde(deserialize_with = "to_rust_decimal_15dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement16 {
    #[serde(deserialize_with = "to_rust_decimal_16dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement17 {
    #[serde(deserialize_with = "to_rust_decimal_17dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement18 {
    #[serde(deserialize_with = "to_rust_decimal_18dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement19 {
    #[serde(deserialize_with = "to_rust_decimal_19dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement20 {
    #[serde(deserialize_with = "to_rust_decimal_20dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement21 {
    #[serde(deserialize_with = "to_rust_decimal_21dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement22 {
    #[serde(deserialize_with = "to_rust_decimal_22dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement23 {
    #[serde(deserialize_with = "to_rust_decimal_23dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement24 {
    #[serde(deserialize_with = "to_rust_decimal_24dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement25 {
    #[serde(deserialize_with = "to_rust_decimal_25dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement26 {
    #[serde(deserialize_with = "to_rust_decimal_26dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement27 {
    #[serde(deserialize_with = "to_rust_decimal_27dp_opt")]
    maybe_num: Option<Decimal>,
}
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement28 {
    #[serde(deserialize_with = "to_rust_decimal_28dp_opt")]
    maybe_num: Option<Decimal>,
}

impl FromStr for OptionalDecimalElement00 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement01 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement02 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement03 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement04 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement05 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement06 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement07 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement08 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement09 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement10 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement11 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement12 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement13 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement14 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement15 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement16 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement17 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement18 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement19 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement20 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement21 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement22 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement23 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement24 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement25 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement26 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement27 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}
impl FromStr for OptionalDecimalElement28 {
    type Err = quick_xml::DeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const TEST_DECIMAL: i64 = i64::MAX;
fn gen_test_xml_num(dec_val: &str) -> String {
    format!("<Test><d:Num>{}</d:Num></Test>", dec_val.to_string())
}
fn gen_test_xml_maybe_num(dec_val: &str) -> String {
    format!(
        "<Test><d:MaybeNum>{}</d:MaybeNum></Test>",
        dec_val.to_string()
    )
}

#[test]
fn should_deserialize_decimal_0dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 0).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement00::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_1dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 1).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement01::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_2dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 2).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement02::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_3dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 3).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement03::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_4dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 4).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement04::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_5dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 5).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement05::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_6dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 6).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement06::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_7dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 7).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement07::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_8dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 8).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement08::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_9dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 9).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement09::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_10dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 10).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement10::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_11dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 11).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement11::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_12dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 12).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement12::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_13dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 13).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement13::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_14dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 14).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement14::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_15dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 15).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement15::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_16dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 16).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement16::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_17dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 17).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement17::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_18dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 18).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement18::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_19dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 19).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement19::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_20dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 20).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement20::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_21dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 21).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement21::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_22dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 22).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement22::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_23dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 23).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement23::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_24dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 24).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement24::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_25dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 25).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement25::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_26dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 26).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement26::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_27dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 27).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement27::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_decimal_28dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 28).unwrap();
    let decimal_xml = &gen_test_xml_num(&dec.to_string());

    match DecimalElement28::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison(&dec, &result.num),
        Err(err) => Err(format!("{err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_optional_decimal_0dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 0).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement00::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_1dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 1).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement01::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_2dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 2).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement02::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_3dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 3).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement03::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_4dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 4).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement04::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_5dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 5).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement05::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_6dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 6).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement06::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_7dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 7).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement07::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_8dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 8).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement08::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_9dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 9).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement09::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_10dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 10).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement10::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_11dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 11).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement11::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_12dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 12).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement12::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_13dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 13).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement13::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_14dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 14).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement14::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_15dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 15).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement15::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_16dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 16).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement16::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_17dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 17).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement17::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_18dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 18).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement18::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_19dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 19).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement19::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_20dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 20).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement20::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_21dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 21).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement21::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_22dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 22).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement22::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_23dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 23).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement23::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_24dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 24).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement24::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_25dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 25).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement25::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_26dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 26).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement26::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_27dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 27).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement27::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}
#[test]
fn should_deserialize_optional_decimal_28dp() -> Result<(), String> {
    let dec = Decimal::try_new(TEST_DECIMAL, 28).unwrap();
    let decimal_xml = &gen_test_xml_maybe_num(&dec.to_string());

    match OptionalDecimalElement28::from_str(decimal_xml) {
        Ok(result) => handle_test_comparison_opt(&Some(dec), &result.maybe_num),
        Err(err) => Err(format!("{err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_parse_decimal_no_decimal_separator() -> Result<(), String> {
    let dec_str = "123".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12300000, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_decimal_separator_missing_fraction() -> Result<(), String> {
    let dec_str = "123.".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12300000, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_decimal_separator_fraction_too_short() -> Result<(), String> {
    let dec_str = "123.45".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12345000, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_decimal_separator_fraction_correct_length() -> Result<(), String> {
    let dec_str = "123.45678".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12345678, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_decimal_separator_missing_integer() -> Result<(), String> {
    let dec_str = ".123".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12300, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_empty_decimal_string() -> Result<(), String> {
    let dec_str = "".to_string();
    let scale: u32 = 3;
    let expected_val = Decimal::try_new(0, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_max_digits() -> Result<(), String> {
    let dec_str = "123456789.0123456789".to_string();
    let scale: u32 = 10;
    let expected_val = Decimal::try_new(1234567890123456789, scale)
        .unwrap()
        .to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_max_i64() -> Result<(), String> {
    let dec_str = "9.223372036854775807".to_string();
    let scale: u32 = 18;
    let expected_val = Decimal::try_new(9223372036854775807, scale)
        .unwrap()
        .to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_fraction_too_long_trailing_zeroes() -> Result<(), String> {
    let dec_str = ".1234500".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12345, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tests to handle non-fatal errors
#[test]
fn should_fail_scale_too_large() -> Result<(), String> {
    let expected_str = "Scale exceeds the maximum precision allowed: 29 > 28".to_string();
    let res = match dec_str_to_rust_decimal("123".to_string(), 29) {
        Ok(val) => val.to_string(),
        Err(err) => err.to_string(),
    };

    handle_test_comparison(&res, &expected_str)
}

#[test]
fn should_fail_empty_decimal_string_scale_too_large() -> Result<(), String> {
    let expected_str = "Scale exceeds the maximum precision allowed: 29 > 28".to_string();
    let res = match dec_str_to_rust_decimal("".to_string(), 29) {
        Ok(val) => val.to_string(),
        Err(err) => err.to_string(),
    };

    handle_test_comparison(&res, &expected_str)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tests should panic due to data loss
#[test]
#[should_panic]
fn should_panic_on_too_many_digits() {
    let _ = dec_str_to_rust_decimal("12345678901234567890".to_string(), 19);
}

#[test]
#[should_panic]
fn should_panic_on_fraction_longer_than_scale() {
    let _ = dec_str_to_rust_decimal("123.12345678".to_string(), 7);
}

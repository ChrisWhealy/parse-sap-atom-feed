use super::*;
use crate::test_utils::*;

use serde::Deserialize;
use std::str::FromStr;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Tests for Decimal deserialization
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct DecimalElement {
    #[serde(deserialize_with = "to_rust_decimal_0dp")]
    some_number_0dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_1dp")]
    some_number_1dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_2dp")]
    some_number_2dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_3dp")]
    some_number_3dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_4dp")]
    some_number_4dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_5dp")]
    some_number_5dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_6dp")]
    some_number_6dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_7dp")]
    some_number_7dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_8dp")]
    some_number_8dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_9dp")]
    some_number_9dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_10dp")]
    some_number_10dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_11dp")]
    some_number_11dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_12dp")]
    some_number_12dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_13dp")]
    some_number_13dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_14dp")]
    some_number_14dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_15dp")]
    some_number_15dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_16dp")]
    some_number_16dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_17dp")]
    some_number_17dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_18dp")]
    some_number_18dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_19dp")]
    some_number_19dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_20dp")]
    some_number_20dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_21dp")]
    some_number_21dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_22dp")]
    some_number_22dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_23dp")]
    some_number_23dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_24dp")]
    some_number_24dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_25dp")]
    some_number_25dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_26dp")]
    some_number_26dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_27dp")]
    some_number_27dp: Decimal,
    #[serde(deserialize_with = "to_rust_decimal_28dp")]
    some_number_28dp: Decimal,
}

impl FromStr for DecimalElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionalDecimalElement {
    #[serde(deserialize_with = "to_rust_decimal_0dp_opt")]
    some_number_0dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_1dp_opt")]
    some_number_1dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_2dp_opt")]
    some_number_2dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_3dp_opt")]
    some_number_3dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_4dp_opt")]
    some_number_4dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_5dp_opt")]
    some_number_5dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_6dp_opt")]
    some_number_6dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_7dp_opt")]
    some_number_7dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_8dp_opt")]
    some_number_8dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_9dp_opt")]
    some_number_9dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_10dp_opt")]
    some_number_10dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_11dp_opt")]
    some_number_11dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_12dp_opt")]
    some_number_12dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_13dp_opt")]
    some_number_13dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_14dp_opt")]
    some_number_14dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_15dp_opt")]
    some_number_15dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_16dp_opt")]
    some_number_16dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_17dp_opt")]
    some_number_17dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_18dp_opt")]
    some_number_18dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_19dp_opt")]
    some_number_19dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_20dp_opt")]
    some_number_20dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_21dp_opt")]
    some_number_21dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_22dp_opt")]
    some_number_22dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_23dp_opt")]
    some_number_23dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_24dp_opt")]
    some_number_24dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_25dp_opt")]
    some_number_25dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_26dp_opt")]
    some_number_26dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_27dp_opt")]
    some_number_27dp_opt: Option<Decimal>,
    #[serde(deserialize_with = "to_rust_decimal_28dp_opt")]
    some_number_28dp_opt: Option<Decimal>,
}

impl FromStr for OptionalDecimalElement {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_decimal() -> Result<(), String> {
    let test_decimal: i64 = i64::MAX;

    let decimal_0dp = Decimal::try_new(test_decimal, 0).unwrap();
    let decimal_1dp = Decimal::try_new(test_decimal, 1).unwrap();
    let decimal_2dp = Decimal::try_new(test_decimal, 2).unwrap();
    let decimal_3dp = Decimal::try_new(test_decimal, 3).unwrap();
    let decimal_4dp = Decimal::try_new(test_decimal, 4).unwrap();
    let decimal_5dp = Decimal::try_new(test_decimal, 5).unwrap();
    let decimal_6dp = Decimal::try_new(test_decimal, 6).unwrap();
    let decimal_7dp = Decimal::try_new(test_decimal, 7).unwrap();
    let decimal_8dp = Decimal::try_new(test_decimal, 8).unwrap();
    let decimal_9dp = Decimal::try_new(test_decimal, 9).unwrap();
    let decimal_10dp = Decimal::try_new(test_decimal, 10).unwrap();
    let decimal_11dp = Decimal::try_new(test_decimal, 11).unwrap();
    let decimal_12dp = Decimal::try_new(test_decimal, 12).unwrap();
    let decimal_13dp = Decimal::try_new(test_decimal, 13).unwrap();
    let decimal_14dp = Decimal::try_new(test_decimal, 14).unwrap();
    let decimal_15dp = Decimal::try_new(test_decimal, 15).unwrap();
    let decimal_16dp = Decimal::try_new(test_decimal, 16).unwrap();
    let decimal_17dp = Decimal::try_new(test_decimal, 17).unwrap();
    let decimal_18dp = Decimal::try_new(test_decimal, 18).unwrap();
    let decimal_19dp = Decimal::try_new(test_decimal, 19).unwrap();
    let decimal_20dp = Decimal::try_new(test_decimal, 20).unwrap();
    let decimal_21dp = Decimal::try_new(test_decimal, 21).unwrap();
    let decimal_22dp = Decimal::try_new(test_decimal, 22).unwrap();
    let decimal_23dp = Decimal::try_new(test_decimal, 23).unwrap();
    let decimal_24dp = Decimal::try_new(test_decimal, 24).unwrap();
    let decimal_25dp = Decimal::try_new(test_decimal, 25).unwrap();
    let decimal_26dp = Decimal::try_new(test_decimal, 26).unwrap();
    let decimal_27dp = Decimal::try_new(test_decimal, 27).unwrap();
    let decimal_28dp = Decimal::try_new(test_decimal, 28).unwrap();

    let decimal_xml = &format!(
        "<Test>\
<d:SomeNumber0dp>{}</d:SomeNumber0dp>\
<d:SomeNumber1dp>{}</d:SomeNumber1dp>\
<d:SomeNumber2dp>{}</d:SomeNumber2dp>\
<d:SomeNumber3dp>{}</d:SomeNumber3dp>\
<d:SomeNumber4dp>{}</d:SomeNumber4dp>\
<d:SomeNumber5dp>{}</d:SomeNumber5dp>\
<d:SomeNumber6dp>{}</d:SomeNumber6dp>\
<d:SomeNumber7dp>{}</d:SomeNumber7dp>\
<d:SomeNumber8dp>{}</d:SomeNumber8dp>\
<d:SomeNumber9dp>{}</d:SomeNumber9dp>\
<d:SomeNumber10dp>{}</d:SomeNumber10dp>\
<d:SomeNumber11dp>{}</d:SomeNumber11dp>\
<d:SomeNumber12dp>{}</d:SomeNumber12dp>\
<d:SomeNumber13dp>{}</d:SomeNumber13dp>\
<d:SomeNumber14dp>{}</d:SomeNumber14dp>\
<d:SomeNumber15dp>{}</d:SomeNumber15dp>\
<d:SomeNumber16dp>{}</d:SomeNumber16dp>\
<d:SomeNumber17dp>{}</d:SomeNumber17dp>\
<d:SomeNumber18dp>{}</d:SomeNumber18dp>\
<d:SomeNumber19dp>{}</d:SomeNumber19dp>\
<d:SomeNumber20dp>{}</d:SomeNumber20dp>\
<d:SomeNumber21dp>{}</d:SomeNumber21dp>\
<d:SomeNumber22dp>{}</d:SomeNumber22dp>\
<d:SomeNumber23dp>{}</d:SomeNumber23dp>\
<d:SomeNumber24dp>{}</d:SomeNumber24dp>\
<d:SomeNumber25dp>{}</d:SomeNumber25dp>\
<d:SomeNumber26dp>{}</d:SomeNumber26dp>\
<d:SomeNumber27dp>{}</d:SomeNumber27dp>\
<d:SomeNumber28dp>{}</d:SomeNumber28dp>\
</Test>",
        decimal_0dp.to_string(),
        decimal_1dp.to_string(),
        decimal_2dp.to_string(),
        decimal_3dp.to_string(),
        decimal_4dp.to_string(),
        decimal_5dp.to_string(),
        decimal_6dp.to_string(),
        decimal_7dp.to_string(),
        decimal_8dp.to_string(),
        decimal_9dp.to_string(),
        decimal_10dp.to_string(),
        decimal_11dp.to_string(),
        decimal_12dp.to_string(),
        decimal_13dp.to_string(),
        decimal_14dp.to_string(),
        decimal_15dp.to_string(),
        decimal_16dp.to_string(),
        decimal_17dp.to_string(),
        decimal_18dp.to_string(),
        decimal_19dp.to_string(),
        decimal_20dp.to_string(),
        decimal_21dp.to_string(),
        decimal_22dp.to_string(),
        decimal_23dp.to_string(),
        decimal_24dp.to_string(),
        decimal_25dp.to_string(),
        decimal_26dp.to_string(),
        decimal_27dp.to_string(),
        decimal_28dp.to_string(),
    );

    match DecimalElement::from_str(decimal_xml) {
        Ok(result) => {
            handle_test_comparison(&decimal_0dp, &result.some_number_0dp)?;
            handle_test_comparison(&decimal_1dp, &result.some_number_1dp)?;
            handle_test_comparison(&decimal_2dp, &result.some_number_2dp)?;
            handle_test_comparison(&decimal_3dp, &result.some_number_3dp)?;
            handle_test_comparison(&decimal_4dp, &result.some_number_4dp)?;
            handle_test_comparison(&decimal_5dp, &result.some_number_5dp)?;
            handle_test_comparison(&decimal_6dp, &result.some_number_6dp)?;
            handle_test_comparison(&decimal_7dp, &result.some_number_7dp)?;
            handle_test_comparison(&decimal_8dp, &result.some_number_8dp)?;
            handle_test_comparison(&decimal_9dp, &result.some_number_9dp)?;
            handle_test_comparison(&decimal_10dp, &result.some_number_10dp)?;
            handle_test_comparison(&decimal_11dp, &result.some_number_11dp)?;
            handle_test_comparison(&decimal_12dp, &result.some_number_12dp)?;
            handle_test_comparison(&decimal_13dp, &result.some_number_13dp)?;
            handle_test_comparison(&decimal_14dp, &result.some_number_14dp)?;
            handle_test_comparison(&decimal_15dp, &result.some_number_15dp)?;
            handle_test_comparison(&decimal_16dp, &result.some_number_16dp)?;
            handle_test_comparison(&decimal_17dp, &result.some_number_17dp)?;
            handle_test_comparison(&decimal_18dp, &result.some_number_18dp)?;
            handle_test_comparison(&decimal_19dp, &result.some_number_19dp)?;
            handle_test_comparison(&decimal_20dp, &result.some_number_20dp)?;
            handle_test_comparison(&decimal_21dp, &result.some_number_21dp)?;
            handle_test_comparison(&decimal_22dp, &result.some_number_22dp)?;
            handle_test_comparison(&decimal_23dp, &result.some_number_23dp)?;
            handle_test_comparison(&decimal_24dp, &result.some_number_24dp)?;
            handle_test_comparison(&decimal_25dp, &result.some_number_25dp)?;
            handle_test_comparison(&decimal_26dp, &result.some_number_26dp)?;
            handle_test_comparison(&decimal_27dp, &result.some_number_27dp)?;
            handle_test_comparison(&decimal_28dp, &result.some_number_28dp)
        }
        Err(err) => Err(format!("{err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_deserialize_optional_decimal() -> Result<(), String> {
    let test_decimal: i64 = i64::MAX;

    let decimal_0dp_opt = Some(Decimal::try_new(test_decimal, 0).unwrap());
    let decimal_1dp_opt = Some(Decimal::try_new(test_decimal, 1).unwrap());
    let decimal_2dp_opt = Some(Decimal::try_new(test_decimal, 2).unwrap());
    let decimal_3dp_opt = Some(Decimal::try_new(test_decimal, 3).unwrap());
    let decimal_4dp_opt = Some(Decimal::try_new(test_decimal, 4).unwrap());
    let decimal_5dp_opt = Some(Decimal::try_new(test_decimal, 5).unwrap());
    let decimal_6dp_opt = Some(Decimal::try_new(test_decimal, 6).unwrap());
    let decimal_7dp_opt = Some(Decimal::try_new(test_decimal, 7).unwrap());
    let decimal_8dp_opt = Some(Decimal::try_new(test_decimal, 8).unwrap());
    let decimal_9dp_opt = Some(Decimal::try_new(test_decimal, 9).unwrap());
    let decimal_10dp_opt = Some(Decimal::try_new(test_decimal, 10).unwrap());
    let decimal_11dp_opt = Some(Decimal::try_new(test_decimal, 11).unwrap());
    let decimal_12dp_opt = Some(Decimal::try_new(test_decimal, 12).unwrap());
    let decimal_13dp_opt = Some(Decimal::try_new(test_decimal, 13).unwrap());
    let decimal_14dp_opt = Some(Decimal::try_new(test_decimal, 14).unwrap());
    let decimal_15dp_opt = Some(Decimal::try_new(test_decimal, 15).unwrap());
    let decimal_16dp_opt = Some(Decimal::try_new(test_decimal, 16).unwrap());
    let decimal_17dp_opt = Some(Decimal::try_new(test_decimal, 17).unwrap());
    let decimal_18dp_opt = Some(Decimal::try_new(test_decimal, 18).unwrap());
    let decimal_19dp_opt = Some(Decimal::try_new(test_decimal, 19).unwrap());
    let decimal_20dp_opt = Some(Decimal::try_new(test_decimal, 20).unwrap());
    let decimal_21dp_opt = Some(Decimal::try_new(test_decimal, 21).unwrap());
    let decimal_22dp_opt = Some(Decimal::try_new(test_decimal, 22).unwrap());
    let decimal_23dp_opt = Some(Decimal::try_new(test_decimal, 23).unwrap());
    let decimal_24dp_opt = Some(Decimal::try_new(test_decimal, 24).unwrap());
    let decimal_25dp_opt = Some(Decimal::try_new(test_decimal, 25).unwrap());
    let decimal_26dp_opt = Some(Decimal::try_new(test_decimal, 26).unwrap());
    let decimal_27dp_opt = Some(Decimal::try_new(test_decimal, 27).unwrap());
    let decimal_28dp_opt = Some(Decimal::try_new(test_decimal, 28).unwrap());

    let decimal_xml = &format!(
        "<Test>\
<d:SomeNumber0dpOpt>{}</d:SomeNumber0dpOpt>\
<d:SomeNumber1dpOpt>{}</d:SomeNumber1dpOpt>\
<d:SomeNumber2dpOpt>{}</d:SomeNumber2dpOpt>\
<d:SomeNumber3dpOpt>{}</d:SomeNumber3dpOpt>\
<d:SomeNumber4dpOpt>{}</d:SomeNumber4dpOpt>\
<d:SomeNumber5dpOpt>{}</d:SomeNumber5dpOpt>\
<d:SomeNumber6dpOpt>{}</d:SomeNumber6dpOpt>\
<d:SomeNumber7dpOpt>{}</d:SomeNumber7dpOpt>\
<d:SomeNumber8dpOpt>{}</d:SomeNumber8dpOpt>\
<d:SomeNumber9dpOpt>{}</d:SomeNumber9dpOpt>\
<d:SomeNumber10dpOpt>{}</d:SomeNumber10dpOpt>\
<d:SomeNumber11dpOpt>{}</d:SomeNumber11dpOpt>\
<d:SomeNumber12dpOpt>{}</d:SomeNumber12dpOpt>\
<d:SomeNumber13dpOpt>{}</d:SomeNumber13dpOpt>\
<d:SomeNumber14dpOpt>{}</d:SomeNumber14dpOpt>\
<d:SomeNumber15dpOpt>{}</d:SomeNumber15dpOpt>\
<d:SomeNumber16dpOpt>{}</d:SomeNumber16dpOpt>\
<d:SomeNumber17dpOpt>{}</d:SomeNumber17dpOpt>\
<d:SomeNumber18dpOpt>{}</d:SomeNumber18dpOpt>\
<d:SomeNumber19dpOpt>{}</d:SomeNumber19dpOpt>\
<d:SomeNumber20dpOpt>{}</d:SomeNumber20dpOpt>\
<d:SomeNumber21dpOpt>{}</d:SomeNumber21dpOpt>\
<d:SomeNumber22dpOpt>{}</d:SomeNumber22dpOpt>\
<d:SomeNumber23dpOpt>{}</d:SomeNumber23dpOpt>\
<d:SomeNumber24dpOpt>{}</d:SomeNumber24dpOpt>\
<d:SomeNumber25dpOpt>{}</d:SomeNumber25dpOpt>\
<d:SomeNumber26dpOpt>{}</d:SomeNumber26dpOpt>\
<d:SomeNumber27dpOpt>{}</d:SomeNumber27dpOpt>\
<d:SomeNumber28dpOpt>{}</d:SomeNumber28dpOpt>\
</Test>",
        decimal_0dp_opt.unwrap().to_string(),
        decimal_1dp_opt.unwrap().to_string(),
        decimal_2dp_opt.unwrap().to_string(),
        decimal_3dp_opt.unwrap().to_string(),
        decimal_4dp_opt.unwrap().to_string(),
        decimal_5dp_opt.unwrap().to_string(),
        decimal_6dp_opt.unwrap().to_string(),
        decimal_7dp_opt.unwrap().to_string(),
        decimal_8dp_opt.unwrap().to_string(),
        decimal_9dp_opt.unwrap().to_string(),
        decimal_10dp_opt.unwrap().to_string(),
        decimal_11dp_opt.unwrap().to_string(),
        decimal_12dp_opt.unwrap().to_string(),
        decimal_13dp_opt.unwrap().to_string(),
        decimal_14dp_opt.unwrap().to_string(),
        decimal_15dp_opt.unwrap().to_string(),
        decimal_16dp_opt.unwrap().to_string(),
        decimal_17dp_opt.unwrap().to_string(),
        decimal_18dp_opt.unwrap().to_string(),
        decimal_19dp_opt.unwrap().to_string(),
        decimal_20dp_opt.unwrap().to_string(),
        decimal_21dp_opt.unwrap().to_string(),
        decimal_22dp_opt.unwrap().to_string(),
        decimal_23dp_opt.unwrap().to_string(),
        decimal_24dp_opt.unwrap().to_string(),
        decimal_25dp_opt.unwrap().to_string(),
        decimal_26dp_opt.unwrap().to_string(),
        decimal_27dp_opt.unwrap().to_string(),
        decimal_28dp_opt.unwrap().to_string(),
    );

    match OptionalDecimalElement::from_str(decimal_xml) {
        Ok(result) => {
            handle_test_comparison_opt(&decimal_0dp_opt, &result.some_number_0dp_opt)?;
            handle_test_comparison_opt(&decimal_1dp_opt, &result.some_number_1dp_opt)?;
            handle_test_comparison_opt(&decimal_2dp_opt, &result.some_number_2dp_opt)?;
            handle_test_comparison_opt(&decimal_3dp_opt, &result.some_number_3dp_opt)?;
            handle_test_comparison_opt(&decimal_4dp_opt, &result.some_number_4dp_opt)?;
            handle_test_comparison_opt(&decimal_5dp_opt, &result.some_number_5dp_opt)?;
            handle_test_comparison_opt(&decimal_6dp_opt, &result.some_number_6dp_opt)?;
            handle_test_comparison_opt(&decimal_7dp_opt, &result.some_number_7dp_opt)?;
            handle_test_comparison_opt(&decimal_8dp_opt, &result.some_number_8dp_opt)?;
            handle_test_comparison_opt(&decimal_9dp_opt, &result.some_number_9dp_opt)?;
            handle_test_comparison_opt(&decimal_10dp_opt, &result.some_number_10dp_opt)?;
            handle_test_comparison_opt(&decimal_11dp_opt, &result.some_number_11dp_opt)?;
            handle_test_comparison_opt(&decimal_12dp_opt, &result.some_number_12dp_opt)?;
            handle_test_comparison_opt(&decimal_13dp_opt, &result.some_number_13dp_opt)?;
            handle_test_comparison_opt(&decimal_14dp_opt, &result.some_number_14dp_opt)?;
            handle_test_comparison_opt(&decimal_15dp_opt, &result.some_number_15dp_opt)?;
            handle_test_comparison_opt(&decimal_16dp_opt, &result.some_number_16dp_opt)?;
            handle_test_comparison_opt(&decimal_17dp_opt, &result.some_number_17dp_opt)?;
            handle_test_comparison_opt(&decimal_18dp_opt, &result.some_number_18dp_opt)?;
            handle_test_comparison_opt(&decimal_19dp_opt, &result.some_number_19dp_opt)?;
            handle_test_comparison_opt(&decimal_20dp_opt, &result.some_number_20dp_opt)?;
            handle_test_comparison_opt(&decimal_21dp_opt, &result.some_number_21dp_opt)?;
            handle_test_comparison_opt(&decimal_22dp_opt, &result.some_number_22dp_opt)?;
            handle_test_comparison_opt(&decimal_23dp_opt, &result.some_number_23dp_opt)?;
            handle_test_comparison_opt(&decimal_24dp_opt, &result.some_number_24dp_opt)?;
            handle_test_comparison_opt(&decimal_25dp_opt, &result.some_number_25dp_opt)?;
            handle_test_comparison_opt(&decimal_26dp_opt, &result.some_number_26dp_opt)?;
            handle_test_comparison_opt(&decimal_27dp_opt, &result.some_number_27dp_opt)?;
            handle_test_comparison_opt(&decimal_28dp_opt, &result.some_number_28dp_opt)
        }
        Err(err) => Err(format!("{err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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
fn should_parse_decimal_fraction_correct_length() -> Result<(), String> {
    let dec_str = "123.45678".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12345678, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
fn should_parse_decimal_missing_mantissa() -> Result<(), String> {
    let dec_str = ".123".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12300, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_fail_scale_too_large() -> Result<(), String> {
    let expected_str = "Scale exceeds the maximum precision allowed: 29 > 28".to_string();
    let res = match Decimal::try_new(i64::MAX, 29) {
        Ok(val) => val.to_string(),
        Err(err) => err.to_string(),
    };

    handle_test_comparison(&res, &expected_str)
}

#[test]
fn should_not_panic_on_fraction_too_long_trailing_zeroes() -> Result<(), String> {
    let dec_str = ".1234500".to_string();
    let scale: u32 = 5;
    let expected_val = Decimal::try_new(12345, scale).unwrap().to_string();

    match dec_str_to_rust_decimal(dec_str, scale as usize) {
        Ok(dec_val) => handle_test_comparison(&dec_val.to_string(), &expected_val),
        Err(err) => Err(err),
    }
}

#[test]
#[should_panic]
fn should_panic_on_fraction_too_long() {
    let dec_str = "123.456789".to_string();
    let scale: u32 = 5;

    let _ = dec_str_to_rust_decimal(dec_str, scale as usize);
}

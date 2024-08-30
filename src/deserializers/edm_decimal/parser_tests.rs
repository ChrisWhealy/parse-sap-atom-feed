use super::*;
use crate::test_utils::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_parse_missing_integer() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits(".0025", 4)?, &25i64)
}
#[test]
fn should_parse_missing_integer_trailing_zeroes() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits(".0025000", 4)?, &25i64)
}
#[test]
fn should_trim_zeroes() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("000123.00000", 2)?, &12300i64)
}
#[test]
fn should_parse_scale_0() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("123", 0)?, &123i64)
}
#[test]
fn should_parse_scale_1() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("123", 1)?, &1_230i64)?;
    handle_test_comparison(&parse_decimal_digits("123.4", 1)?, &1_234i64)
}
#[test]
fn should_parse_scale_3() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("123", 3)?, &123_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123.45", 3)?, &123_450i64)
}
#[test]
fn should_parse_scale_5() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("123", 5)?, &12_300_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123.456", 5)?, &12_345_600i64)
}
#[test]
fn should_parse_scale_7() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("123", 7)?, &1_230_000_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123.456789", 7)?, &1_234_567_890i64,
    )
}
#[test]
fn should_parse_scale_9() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("123", 9)?, &123_000_000_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123.456789", 9)?, &123_456_789_000i64)
}
#[test]
fn should_parse_scale_11() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("123", 11)?, &12_300_000_000_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123.456789123", 11)?, &12_345_678_912_300i64)
}
#[test]
fn should_parse_scale_13() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("123", 13)?, &1_230_000_000_000_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123.456789123", 13)?, &1_234_567_891_230_000i64)
}
#[test]
fn should_parse_scale_15() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.00123", 15)?, &1_230_000_000_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123", 15)?, &123_000_000_000_000_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123.456789123", 15)?, &123_456_789_123_000_000i64)
}
#[test]
fn should_parse_scale_17() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.0000000123", 17)?, &1_230_000_000i64)
}
#[test]
fn should_parse_scale_19() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.000000000123", 19)?, &1_230_000_000i64)
}
#[test]
fn should_parse_scale_21() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.00000000000123", 21)?, &1_230_000_000i64)
}
#[test]
fn should_parse_scale_23() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.0000000000000123", 23)?, &1_230_000_000i64)
}
#[test]
fn should_parse_scale_25() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.000000000000000123", 25)?, &1_230_000_000i64)
}
#[test]
fn should_parse_scale_27() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.00000000000000000123", 27)?, &1_230_000_000i64)
}
#[test]
fn should_parse_scale_28() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.000000000000000000123", 28)?, &1_230_000_000i64)
}
#[test]
fn should_not_pad_beyond_19() -> Result<(), String> {
    handle_test_comparison(&parse_decimal_digits("0.00000123", 16)?, &12_300_000_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123", 16)?, &1_230_000_000_000_000_000i64)?;
    handle_test_comparison(&parse_decimal_digits("123.456789123", 16)?, &1_234_567_891_230_000_000i64)
}

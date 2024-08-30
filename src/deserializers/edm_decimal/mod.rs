use rust_decimal::Decimal;
use serde::{de::Error, Deserialize, Deserializer};
use std::str::FromStr;

// i64::MAX is 19 digits long
static MAX_DIGITS: usize = 19;
// Value duplicated from rust_decimal::constants::MAX_PRECISION which is private...
const MAX_PRECISION: usize = 28;
// Maximum potential padding is 27 zeroes for 0.0000000000000000000000000001
static ZEROES: &str = "000000000000000000000000000";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Parse the digits in the decimal string and return an i64 suitable for creating a new rust_decimal::Decimal value
///
/// * Split integer and fractional parts
/// * Trim trailing zeroes from fractional part before deciding if it's too long
/// * Check the number of fractional digits <= scale
/// * Concatenate integer with zero padded fraction (on the right) then strip leading zeroes
/// * Check the number digits <= MAX_DIGITS
/// * Convert digits to i64
fn parse_decimal_digits(dec_str: &str, scale: usize) -> Result<i64, String> {
    // Just in case it goes horribly wrong...
    let data_loss_prefix = format!(
        "'{}' cannot be converted to rust_decimal::Decimal without loss of data:",
        dec_str,
    );
    // Split string at decimal point (that might not be there)
    let num_parts = dec_str
        .split(".")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let integers = num_parts[0].as_str();
    let fraction = if num_parts.len() == 2 {
        // Trailing zeroes in the fraction are not needed at this point in time
        // However, some trailing zeroes might need to be reinstated before converting to i64
        num_parts[1].trim_end_matches("0")
    } else {
        // Fill the missing fractional zeroes
        &format!("{ZEROES:.*}", scale)
    };

    // Return an error if there are more fractional digits than are permitted by the scale
    if fraction.len() > scale {
        return Err(format!(
            "{} {} fractional digit{} supplied, but scale only permits {}",
            data_loss_prefix,
            fraction.len(),
            if fraction.len() > 1 { "s" } else { "" },
            scale
        ));
    }

    // Zero pad any missing fractional zeroes on the right, combine parts then strip leading zeroes
    let digits = format!("{integers}{:0<scale$}", fraction);
    let digits = digits.trim_start_matches("0");
    let digit_count = digits.len();

    // Can this number be converted to an i64?
    if digit_count > MAX_DIGITS {
        Err(format!(
            "{} Too many digits ({}) to fit in an i64 ({})",
            data_loss_prefix, digit_count, MAX_DIGITS
        ))
    } else {
        Ok(i64::from_str(&format!("{digits}")).unwrap())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// String -> rust_decimal::Decimal parser
fn dec_str_to_rust_decimal(dec_str: String, scale: usize) -> Result<Decimal, String> {
    if scale > MAX_PRECISION {
        return Err(format!(
            "Scale exceeds the maximum precision allowed: {} > {}",
            scale, MAX_PRECISION
        ));
    }

    // Check for cases that always return zero
    if dec_str.is_empty()
        || dec_str.eq("0")
        || dec_str.eq("0.0")
        || dec_str.eq("0.")
        || dec_str.eq(".0")
    {
        return Ok(Decimal::new(0, scale as u32));
    }

    // If parse_decimal_digits returns an error, then data loss has been detected - throw toys out of pram
    let digits_i64 = match parse_decimal_digits(&dec_str, scale) {
        Ok(digits) => digits,
        Err(err) => panic!("{}", err),
    };

    Decimal::try_new(digits_i64, scale as u32).or_else(|err| Err(err.to_string()))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn to_rust_decimal_inner<'de, D>(deserializer: D, scale: u32) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(dec_str) => {
            dec_str_to_rust_decimal(dec_str, scale as usize).or_else(|err| Err(Error::custom(err)))
        }
        Err(err) => Err(Error::custom(err.to_string())),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn to_rust_decimal_0dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_0dp(deserializer).ok())
}
pub fn to_rust_decimal_1dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_1dp(deserializer).ok())
}
pub fn to_rust_decimal_2dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_2dp(deserializer).ok())
}
pub fn to_rust_decimal_3dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_3dp(deserializer).ok())
}
pub fn to_rust_decimal_4dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_4dp(deserializer).ok())
}
pub fn to_rust_decimal_5dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_5dp(deserializer).ok())
}
pub fn to_rust_decimal_6dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_6dp(deserializer).ok())
}
pub fn to_rust_decimal_7dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_7dp(deserializer).ok())
}
pub fn to_rust_decimal_8dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_8dp(deserializer).ok())
}
pub fn to_rust_decimal_9dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_9dp(deserializer).ok())
}
pub fn to_rust_decimal_10dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_10dp(deserializer).ok())
}
pub fn to_rust_decimal_11dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_11dp(deserializer).ok())
}
pub fn to_rust_decimal_12dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_12dp(deserializer).ok())
}
pub fn to_rust_decimal_13dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_13dp(deserializer).ok())
}
pub fn to_rust_decimal_14dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_14dp(deserializer).ok())
}
pub fn to_rust_decimal_15dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_15dp(deserializer).ok())
}
pub fn to_rust_decimal_16dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_16dp(deserializer).ok())
}
pub fn to_rust_decimal_17dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_17dp(deserializer).ok())
}
pub fn to_rust_decimal_18dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_18dp(deserializer).ok())
}
pub fn to_rust_decimal_19dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_19dp(deserializer).ok())
}
pub fn to_rust_decimal_20dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_20dp(deserializer).ok())
}
pub fn to_rust_decimal_21dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_21dp(deserializer).ok())
}
pub fn to_rust_decimal_22dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_22dp(deserializer).ok())
}
pub fn to_rust_decimal_23dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_23dp(deserializer).ok())
}
pub fn to_rust_decimal_24dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_24dp(deserializer).ok())
}
pub fn to_rust_decimal_25dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_25dp(deserializer).ok())
}
pub fn to_rust_decimal_26dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_26dp(deserializer).ok())
}
pub fn to_rust_decimal_27dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_27dp(deserializer).ok())
}
pub fn to_rust_decimal_28dp_opt<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(to_rust_decimal_28dp(deserializer).ok())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn to_rust_decimal_0dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 0)
}
pub fn to_rust_decimal_1dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 1)
}
pub fn to_rust_decimal_2dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 2)
}
pub fn to_rust_decimal_3dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 3)
}
pub fn to_rust_decimal_4dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 4)
}
pub fn to_rust_decimal_5dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 5)
}
pub fn to_rust_decimal_6dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 6)
}
pub fn to_rust_decimal_7dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 7)
}
pub fn to_rust_decimal_8dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 8)
}
pub fn to_rust_decimal_9dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 9)
}
pub fn to_rust_decimal_10dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 10)
}
pub fn to_rust_decimal_11dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 11)
}
pub fn to_rust_decimal_12dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 12)
}
pub fn to_rust_decimal_13dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 13)
}
pub fn to_rust_decimal_14dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 14)
}
pub fn to_rust_decimal_15dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 15)
}
pub fn to_rust_decimal_16dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 16)
}
pub fn to_rust_decimal_17dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 17)
}
pub fn to_rust_decimal_18dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 18)
}
pub fn to_rust_decimal_19dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 19)
}
pub fn to_rust_decimal_20dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 20)
}
pub fn to_rust_decimal_21dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 21)
}
pub fn to_rust_decimal_22dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 22)
}
pub fn to_rust_decimal_23dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 23)
}
pub fn to_rust_decimal_24dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 24)
}
pub fn to_rust_decimal_25dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 25)
}
pub fn to_rust_decimal_26dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 26)
}
pub fn to_rust_decimal_27dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 27)
}
pub fn to_rust_decimal_28dp<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    to_rust_decimal_inner(deserializer, 28)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod parser_tests;
#[cfg(test)]
mod unit_tests;

use rust_decimal::Decimal;
use serde::{de::Error, Deserialize, Deserializer};
use std::str::FromStr;

// Maximum potential padding is 28 zeroes
static ZEROES: &str = "0000000000000000000000000000";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Transforming a decimal string value requires the following steps:
/// 1. Split the string at the decimal point
/// 1. Check that the number of digits to the right of the decimal point matches the value of the scale property.
///     * If too few, then zero pad
///     * If too many, then there's something badly wrong with the data coming out of the OData system. Panic to avoid data loss
/// 1. Convert the digits to an i64
/// 1. Convert the i64 to a Decimal using the appropriate scale value
fn dec_str_to_rust_decimal(dec_str: String, scale: usize) -> Result<Decimal, String> {
    let mut digit_parts = dec_str
        .split(".")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    // The decimal point might not be present in the string E.G. "123"
    if digit_parts.len() == 1 {
        // Fill in the missing decimal digits
        digit_parts.push(format!("{ZEROES:.*}", scale));
    } else if digit_parts.len() == 2 {
        // Do the decimal place digits need zero padding on the right?
        if digit_parts[1].len() < scale {
            digit_parts[1] = format!("{:0<scale$}", digit_parts[1]);
        } else if digit_parts[1].len() > scale {
            // Before panicking, first remove any trailing zeroes as this does not constitute data loss
            let trimmed_fraction = digit_parts[1].trim_end_matches("0");

            if trimmed_fraction.len() > scale {
                // Well this is bad... Something seems to have gone wrong in the OData service as it is outputting more
                // decimal digits have been defined by the metadata type definition's scale attribute.
                // Have to panic here to avoid data loss...
                panic!(
                    "Data loss: Edm.Decimal value {} contains too many fractional digits. Expected {} but got {}",
                    dec_str, scale, digit_parts[1].len()
                );
            } else {
                digit_parts[1] = trimmed_fraction.to_string();
            }
        }
    }

    let digits = digit_parts.join("");
    let i64_digits = i64::from_str(&digits).or_else(|err| Err(err.to_string()))?;
    Decimal::try_new(i64_digits, scale as u32).or_else(|err| Err(err.to_string()))
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

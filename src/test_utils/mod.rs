use std::fmt::{Debug, Display};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn handle_test_bool(received_bool: bool) -> Result<(), String> {
    if received_bool {
        Ok(())
    } else {
        Err("Expected Boolean True.  Got Boolean False instead".to_string())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn handle_test_comparison<T>(received_val: &T, expected_val: &T) -> Result<(), String>
where
    T: PartialEq + Clone + Debug + Display,
{
    if received_val.eq(expected_val) {
        Ok(())
    } else {
        Err(format!(
            "Expected '{}'.  Got '{}' instead",
            expected_val, received_val
        ))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn handle_test_comparison_opt<T>(
    received_val: &Option<T>,
    expected_val: &Option<T>,
) -> Result<(), String>
where
    T: PartialEq + Clone + Debug + Display,
{
    if received_val.is_none() && expected_val.is_none() {
        Ok(())
    } else if received_val.is_some() && expected_val.is_some() {
        handle_test_comparison(
            &received_val.clone().unwrap(),
            &expected_val.clone().unwrap(),
        )
    } else {
        Err(format!(
            "Can't compare {:?} with {:?}.",
            received_val, expected_val
        ))
    }
}

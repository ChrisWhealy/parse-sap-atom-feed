use super::*;
use crate::test_utils::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_correct_bad_etag_raw() -> Result<(), String> {
    let result = sanitise_bad_etags(
        "<entry m:etag=\"W/\"datetime'2023-08-31T01%3A48%3A52.9972620'\"\">".to_string(),
    );
    let expected_result =
        "<entry m:etag=\"datetime'2023-08-31T01%3A48%3A52.9972620'\">".to_string();
    handle_test_comparison(&result, &expected_result)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_correct_bad_etag_encoded()  -> Result<(), String> {
    let result = sanitise_bad_etags(
        "<entry m:etag=\"W/&quot;datetime'2023-08-31T01%3A48%3A52.9972620'&quot;\">".to_string(),
    );
    let expected_result =
        "<entry m:etag=\"datetime'2023-08-31T01%3A48%3A52.9972620'\">".to_string();
    handle_test_comparison(&result, &expected_result)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_replace_naked_ampersand_no_whitespace()  -> Result<(), String> {
    let result = sanitise_naked_ampersand("<d:Landx>St Kitts&Nevis</d:Landx>".to_string());
    let expected_result ="<d:Landx>St Kitts&amp;Nevis</d:Landx>".to_string();
    handle_test_comparison(&result, &expected_result)
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_replace_naked_ampersand_whitespace()  -> Result<(), String> {
    let result = sanitise_naked_ampersand("<d:Category>PDAs & Organizers</d:Category>".to_string());
    let expected_result ="<d:Category>PDAs &amp; Organizers</d:Category>".to_string();
    handle_test_comparison(&result, &expected_result)
}

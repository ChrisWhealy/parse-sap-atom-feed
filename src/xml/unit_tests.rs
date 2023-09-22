use crate::xml::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_correct_bad_etag_raw() {
    assert_eq!(
        sanitise_bad_etags(String::from(
            "<entry m:etag=\"W/\"datetime'2023-08-31T01%3A48%3A52.9972620'\"\">"
        )),
        "<entry m:etag=\"datetime'2023-08-31T01%3A48%3A52.9972620'\">"
    );
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_correct_bad_etag_encoded() {
    assert_eq!(
        sanitise_bad_etags(String::from(
            "<entry m:etag=\"W/&quot;datetime'2023-08-31T01%3A48%3A52.9972620'&quot;\">"
        )),
        "<entry m:etag=\"datetime'2023-08-31T01%3A48%3A52.9972620'\">"
    );
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_replace_naked_ampersand_no_whitespace() {
    assert_eq!(
        sanitise_naked_ampersand(String::from("<d:Landx>St Kitts&Nevis</d:Landx>")),
        "<d:Landx>St Kitts&amp;Nevis</d:Landx>"
    );
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn should_replace_naked_ampersand_whitespace() {
    assert_eq!(
        sanitise_naked_ampersand(String::from("<d:Category>PDAs & Organizers</d:Category>")),
        "<d:Category>PDAs &amp; Organizers</d:Category>"
    );
}

use regex::Regex;

pub static XML_DECLARATION: &[u8] = "<?xml version=\"1.0\" encoding=\"utf-8\"?>".as_bytes();

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// XML Defaults
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}
pub fn default_xml_language() -> String {
    "en".to_string()
}
pub fn default_xml_namespace() -> String {
    "http://schemas.microsoft.com/ado/2008/09/edm".to_string()
}
pub fn default_xml_namespace_app() -> String {
    "http://www.w3.org/2007/app".to_string()
}
pub fn default_xml_namespace_atom() -> Option<String> {
    Some("http://www.w3.org/2005/Atom".to_string())
}
pub fn default_xml_namespace_edmx() -> String {
    "http://schemas.microsoft.com/ado/2007/06/edmx".to_string()
}
pub fn default_xml_namespace_d() -> String {
    "http://schemas.microsoft.com/ado/2007/08/dataservices".to_string()
}
pub fn default_xml_namespace_m() -> String {
    "http://schemas.microsoft.com/ado/2007/08/dataservices/metadata".to_string()
}
pub fn default_xml_namespace_oasis() -> String {
    "http://docs.oasis-open.org/odata/ns/edm".to_string()
}
pub fn default_xml_namespace_sap() -> String {
    "http://www.sap.com/Protocols/SAPData".to_string()
}

/// # CORRECT FORMATTING ERRORS IN RAW XML
///
/// When reading certain entity sets from SAP's demo OData service `GWSAMPLE_BASIC`, various formatting errors have been
/// noticed that will cause an XML parser to throw its toys out of the pram.
///
/// Whether these errors also occur in other SAP-delivered OData services has not been determined; however, coding is
/// included here to correct those errors detected so far in GWSAMPLE_BASIC
///
/// 1. Correct potentially invalid `m:etag` attribute values on an `<entry>` tag:
///
///    ```xml
///    <entry m:etag="W/"datetime'2023-08-31T01%3A00%3A06.0000000'"">
///    ```
///
///    Is corrected to:
///
///    ```xml
///    <entry m:etag="datetime'2023-08-31T01%3A00%3A06.0000000'">
///    ```
///
/// 1. Entity set content properties containing text descriptions are not enclosed in double quotes, neither is it the
///    convention to escape or character encode special characters.
///    E.G.:
///
///    ```xml
///    <d:Category>PDAs & Organizers</d:Category>
///    ```
///
///    Is corrected to:
///
///    ```xml
///    <d:Category>PDAs &amp; Organizers</d:Category>
///    ```
pub fn sanitise_xml(xml: String) -> String {
    let clean_xml = sanitise_bad_etags(xml);

    sanitise_naked_ampersand(clean_xml)
}

fn sanitise_bad_etags(xml: String) -> String {
    if xml.contains("entry m:etag=\"W/\"") || xml.contains("entry m:etag=\"W/&quot;") {
        let mut clean_xml = xml.replace("m:etag=\"W/\"", "m:etag=\"");
        clean_xml = clean_xml.replace("m:etag=\"W/&quot;", "m:etag=\"");
        clean_xml = clean_xml.replace("'\"\">", "'\">");
        clean_xml = clean_xml.replace("'&quot;\">", "'\">");
        clean_xml
    } else {
        xml
    }
}

/// Naked ampersand characters might occur in OData properties containing text descriptions.
/// E.G.:
///
/// ```
/// <d:Category>PDAs & Organizers</d:Category>
/// ```
///
/// Such characters must be replaced with the character encoding `&amp;`
///
/// First, search for ampersands with non-whitespace characters immediately before and after,
/// then search for ampersand characters with a space on either side.
///
/// This functionality assumes that the character string `&amp;` does not occur in the XML
fn sanitise_naked_ampersand(xml: String) -> String {
    let re = Regex::new(r"(\S)\&(\S)").unwrap();
    let clean_xml = re.replace_all(&xml, "$1&amp;$2");

    clean_xml.replace(" & ", " &amp; ")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

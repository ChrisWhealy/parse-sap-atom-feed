pub mod edm_datetime;
pub mod edm_decimal;
pub mod edm_string;

static ONE: &str = "1";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn default_sap_content_version() -> String {
    ONE.to_string()
}
pub fn default_true() -> bool {
    true
}
pub fn default_false() -> bool {
    false
}

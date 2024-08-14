use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::ErrorResolution;
use crate::test_utils::*;

impl std::str::FromStr for ErrorResolution {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_error_resolution() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/error_resolution.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let err_res = ErrorResolution::from_str(&xml).unwrap();

            handle_test_bool(
                err_res
                    .sap_transaction
                    .starts_with("For backend administrators"),
            )?;
            handle_test_bool(err_res.sap_note.starts_with("See SAP Note 1797736"))
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

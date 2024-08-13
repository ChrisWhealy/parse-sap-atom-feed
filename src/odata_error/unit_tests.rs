use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::ODataError;
use crate::test_utils::*;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_error_with_details() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/error_with_details.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let err = ODataError::from_str(&xml).unwrap();

            handle_test_comparison(&err.code, &"/IWBEP/CM_MGW_RT/021".to_string())?;
            handle_test_comparison(
                &err.message,
                &"Method 'SOME_TYPE_GET_ENTITYSET' not implemented in data provider class"
                    .to_string(),
            )
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_error_without_details() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/error_without_details.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let err = ODataError::from_str(&xml).unwrap();

            handle_test_comparison(&err.code, &"/IWFND/MED/170".to_string())?;
            handle_test_comparison(
                &err.message,
                &"No service found for namespace '', name 'ZCUSTOM_SRV', version '0001'"
                    .to_string(),
            )
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

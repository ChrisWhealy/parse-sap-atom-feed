use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::ErrorDetail;
use crate::test_utils::*;

impl std::str::FromStr for ErrorDetail {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_error_detail() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/error_detail.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let empty_str_opt = Some("".to_string());
            let err_det = ErrorDetail::from_str(&xml).unwrap();

            handle_test_comparison_opt(&err_det.content_id, &Some(String::from("")))?;
            handle_test_comparison(&err_det.code, &"/IWBEP/CX_MGW_NOT_IMPL_EXC".to_string())?;
            handle_test_bool(err_det.message.starts_with("Method 'SOME_TYPE_GET_ENTITYSET'"))?;
            handle_test_comparison_opt(&err_det.property_ref, &empty_str_opt)?;
            handle_test_comparison(&err_det.severity, &"error".to_string())?;
            handle_test_comparison_opt(&err_det.target, &empty_str_opt)?;
            handle_test_bool(!err_det.transition)
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

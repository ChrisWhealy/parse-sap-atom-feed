use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::ErrorDetail;

impl std::str::FromStr for ErrorDetail {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_error_detail() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/error_detail.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let err_det = ErrorDetail::from_str(&xml).unwrap();

            assert_eq!(err_det.content_id, Some(String::from("")));
            assert_eq!(err_det.code, String::from("/IWBEP/CX_MGW_NOT_IMPL_EXC"));
            assert!(err_det
                .message
                .starts_with("Method 'SOME_TYPE_GET_ENTITYSET'"));
            assert_eq!(err_det.property_ref, Some(String::from("")));
            assert_eq!(err_det.severity, String::from("error"));
            assert_eq!(err_det.target, Some(String::from("")));
            assert_eq!(err_det.transition, false);
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

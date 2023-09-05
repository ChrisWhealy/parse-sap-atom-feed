use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::InnerError;

impl std::str::FromStr for InnerError {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[test]
pub fn should_parse_inner_error() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/inner_error.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let inner_err = InnerError::from_str(&xml).unwrap();

            assert_eq!(inner_err.transaction_id, "AE181B240AA70000E006489348B6C463");
            assert_eq!(inner_err.timestamp, "20230905123946.1330410");
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

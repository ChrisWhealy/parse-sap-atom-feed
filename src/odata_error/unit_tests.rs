use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::ODataError;

#[test]
pub fn should_parse_error() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/error.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let err = ODataError::from_str(&xml).unwrap();

            assert_eq!(err.code, "/IWBEP/CM_MGW_RT/021");
            assert!(err.message.starts_with("Method 'SOME_TYPE_GET_ENTITYSET'"));
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

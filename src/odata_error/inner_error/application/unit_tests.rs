use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::Application;

impl std::str::FromStr for Application {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_application() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/application.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let app = Application::from_str(&xml).unwrap();

            assert_eq!(app.component_id, Some(String::from("")));
            assert_eq!(app.service_namespace, "/SAP/");
            assert_eq!(app.service_id, "ZCUSTOM_SRV");
            assert_eq!(app.service_version, "0001");
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

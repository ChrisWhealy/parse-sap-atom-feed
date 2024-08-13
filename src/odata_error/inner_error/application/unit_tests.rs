use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::Application;
use crate::test_utils::*;

impl std::str::FromStr for Application {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_application() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/application.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let app = Application::from_str(&xml).unwrap();

            handle_test_comparison_opt(&app.component_id, &Some(String::from("")))?;
            handle_test_comparison(&app.service_namespace, &"/SAP/".to_string())?;
            handle_test_comparison(&app.service_id, &"ZCUSTOM_SRV".to_string())?;
            handle_test_comparison(&app.service_version, &"0001".to_string())
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

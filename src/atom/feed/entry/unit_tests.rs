use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use super::Entry;
use crate::{xml::default_xml_data_services_scheme, test_utils::*};

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DummyService {}

impl FromStr for DummyService {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_atom_entry() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/atom_entry.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let url = "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwfnd/catalogservice;v=2/"
                .to_string();
            let collection = "ServiceCollection('ZSEPMRA_GR_POST_0001')".to_string();
            let entry = Entry::<DummyService>::from_str(&xml).unwrap();

            handle_test_comparison(&entry.id, &format!("{}{}", url, collection))?;
            handle_test_comparison(&entry.title, &collection)?;
            handle_test_comparison(&entry.updated, &"2024-06-18T11:29:35Z".to_string())?;
            handle_test_comparison(&entry.links.len(), &(4usize))?;
            handle_test_bool(!entry.category.fixed)?;
            handle_test_comparison_opt(&entry.category.label, &None)?;
            handle_test_comparison(&entry.category.scheme, &default_xml_data_services_scheme())?;
            handle_test_comparison(&entry.category.term, &"catalogservice.Service".to_string())?;
            handle_test_comparison_opt(
                &entry.content.content_type,
                &Some(String::from("application/xml")),
            )
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

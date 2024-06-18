use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::deserializers::default_xml_dataservices_scheme;

use super::Entry;

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DummyService {}

impl std::str::FromStr for DummyService {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_atom_entry() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/atom_entry.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let entry = Entry::<DummyService>::from_str(&xml).unwrap();

            assert_eq!(
                entry.id,
                "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwfnd/catalogservice;v=2/ServiceCollection('ZSEPMRA_GR_POST_0001')"
            );
            assert_eq!(entry.title, "ServiceCollection('ZSEPMRA_GR_POST_0001')");
            assert_eq!(entry.updated, "2024-06-18T11:29:35Z");
            assert_eq!(entry.links.len(), 4);
            assert_eq!(entry.category.fixed, false);
            assert_eq!(entry.category.label, None);
            assert_eq!(entry.category.scheme, default_xml_dataservices_scheme());
            assert_eq!(entry.category.term, "catalogservice.Service");
            assert_eq!(
                entry.content.content_type,
                Some(String::from("application/xml"))
            );
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

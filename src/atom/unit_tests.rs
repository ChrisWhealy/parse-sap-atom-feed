use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

use super::{AtomLink, AtomService, AtomWorkspace};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_atom_link() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/atom_link.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let atom_link = AtomLink::from_str(&xml).unwrap();

            assert_eq!(
                atom_link.xml_namespace_atom,
                Some(String::from("http://www.w3.org/2005/Atom"))
            );
            assert_eq!(atom_link.rel, "latest-version");
            assert_eq!(
                atom_link.href,
                "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/$metadata"
            );
            assert_eq!(atom_link.title, None);
            assert_eq!(atom_link.mime_type, None);
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_atom_workspace() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/atom_workspace.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let atom_ws = AtomWorkspace::from_str(&xml).unwrap();

            assert_eq!(atom_ws.title, "Data");
            assert_eq!(atom_ws.collections.len(), 16);

            assert_eq!(atom_ws.collections[0].is_creatable, true);
            assert_eq!(atom_ws.collections[0].is_updatable, true);
            assert_eq!(atom_ws.collections[0].is_deletable, true);
            assert_eq!(atom_ws.collections[0].is_searchable, false);
            assert_eq!(atom_ws.collections[0].is_pageable, true);
            assert_eq!(atom_ws.collections[0].content_version, "1");
            assert_eq!(atom_ws.collections[0].href, "BusinessPartnerSet");
            assert_eq!(atom_ws.collections[0].title, "BusinessPartnerSet");
            assert_eq!(atom_ws.collections[0].member_title, "BusinessPartner");

            assert_eq!(atom_ws.collections[5].is_creatable, false);
            assert_eq!(atom_ws.collections[5].is_updatable, false);
            assert_eq!(atom_ws.collections[5].is_deletable, false);
            assert_eq!(atom_ws.collections[5].is_searchable, false);
            assert_eq!(atom_ws.collections[5].is_pageable, false);
            assert_eq!(atom_ws.collections[5].content_version, "1");
            assert_eq!(atom_ws.collections[5].href, "VH_SexSet");
            assert_eq!(atom_ws.collections[5].title, "VH_SexSet");
            assert_eq!(atom_ws.collections[5].member_title, "VH_Sex");
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_atom_service() {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new("./test_data/atom_service.xml")).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let atom_srv = AtomService::from_str(&xml).unwrap();

            assert_eq!(atom_srv.namespace_app, "http://www.w3.org/2007/app");
            assert_eq!(
                atom_srv.namespace_atom,
                Some(String::from("http://www.w3.org/2005/Atom"))
            );
            assert_eq!(
                atom_srv.namespace_m,
                "http://schemas.microsoft.com/ado/2007/08/dataservices/metadata"
            );
            assert_eq!(
                atom_srv.namespace_sap,
                "http://www.sap.com/Protocols/SAPData"
            );
            assert_eq!(
                atom_srv.base_url,
                "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/"
            );
            assert_eq!(atom_srv.language, "en");

            assert_eq!(atom_srv.workspace.collections.len(), 16);
            assert_eq!(atom_srv.links.len(), 2);

            assert_eq!(atom_srv.links[0].rel, "self");
            assert_eq!(
                atom_srv.links[0].href,
                "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/"
            );

            assert_eq!(atom_srv.links[1].rel, "latest-version");
            assert_eq!(
                atom_srv.links[1].href,
                "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/"
            );
        }
        Err(err) => println!("XML test data was not in UTF8 format: {}", err),
    };
}

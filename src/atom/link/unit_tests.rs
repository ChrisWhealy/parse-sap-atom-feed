use super::AtomLink;
use crate::{
    atom::{service::AtomService, workspace::AtomWorkspace},
    test_utils::*,
    xml::{
        default_xml_namespace_app, default_xml_namespace_atom, default_xml_namespace_m,
        default_xml_namespace_sap,
    },
};

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

static PATH_TO_ATOM_WORKSPACE: &str = "./test_data/atom_workspace.xml";
static PATH_TO_ATOM_LINK: &str = "./test_data/atom_link.xml";
static PATH_TO_ATOM_SERVICE: &str = "./test_data/atom_service.xml";
static SRVC_URL: &str = "https://SAPES5.SAPDEVCENTER.COM:443/sap/opu/odata/iwbep/GWSAMPLE_BASIC/";

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_atom_link() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(PATH_TO_ATOM_LINK)).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let base_url = format!("{SRVC_URL}$metadata");
            let atom_link = AtomLink::from_str(&xml).unwrap();

            handle_test_comparison_opt(
                &atom_link.xml_namespace_atom,
                &default_xml_namespace_atom(),
            )?;
            handle_test_comparison(&atom_link.rel, &"latest-version".to_string())?;
            handle_test_comparison(&atom_link.href, &base_url)?;
            handle_test_comparison_opt(&atom_link.title, &None)?;
            handle_test_comparison_opt(&atom_link.mime_type, &None)
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_atom_workspace() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(PATH_TO_ATOM_WORKSPACE)).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let atom_ws = AtomWorkspace::from_str(&xml).unwrap();

            handle_test_comparison(&atom_ws.title, &"Data".to_string())?;
            handle_test_comparison(&atom_ws.collections.len(), &(16usize))?;
            handle_test_bool(atom_ws.collections[0].is_creatable)?;
            handle_test_bool(atom_ws.collections[0].is_updatable)?;
            handle_test_bool(atom_ws.collections[0].is_deletable)?;
            handle_test_bool(!atom_ws.collections[0].is_searchable)?;
            handle_test_bool(atom_ws.collections[0].is_pageable)?;
            handle_test_comparison(&atom_ws.collections[0].content_version, &"1".to_string())?;
            handle_test_comparison(
                &atom_ws.collections[0].href,
                &"BusinessPartnerSet".to_string(),
            )?;
            handle_test_comparison(
                &atom_ws.collections[0].title,
                &"BusinessPartnerSet".to_string(),
            )?;
            handle_test_comparison(
                &atom_ws.collections[0].member_title,
                &"BusinessPartner".to_string(),
            )?;
            handle_test_bool(!atom_ws.collections[5].is_creatable)?;
            handle_test_bool(!atom_ws.collections[5].is_updatable)?;
            handle_test_bool(!atom_ws.collections[5].is_deletable)?;
            handle_test_bool(!atom_ws.collections[5].is_searchable)?;
            handle_test_bool(!atom_ws.collections[5].is_pageable)?;
            handle_test_comparison(&atom_ws.collections[5].content_version, &"1".to_string())?;
            handle_test_comparison(&atom_ws.collections[5].href, &"VH_SexSet".to_string())?;
            handle_test_comparison(&atom_ws.collections[5].title, &"VH_SexSet".to_string())?;
            handle_test_comparison(&atom_ws.collections[5].member_title, &"VH_Sex".to_string())
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
pub fn should_parse_atom_service() -> Result<(), String> {
    let mut xml_buffer: Vec<u8> = Vec::new();
    let test_data = File::open(Path::new(PATH_TO_ATOM_SERVICE)).unwrap();
    let _file_size = BufReader::new(test_data).read_to_end(&mut xml_buffer);

    match String::from_utf8(xml_buffer) {
        Ok(xml) => {
            let base_url = SRVC_URL.to_string();
            let atom_srv = AtomService::from_str(&xml).unwrap();

            handle_test_comparison(&atom_srv.namespace_app, &default_xml_namespace_app())?;
            handle_test_comparison_opt(&atom_srv.namespace_atom, &default_xml_namespace_atom())?;
            handle_test_comparison(&atom_srv.namespace_m, &default_xml_namespace_m())?;
            handle_test_comparison(&atom_srv.namespace_sap, &default_xml_namespace_sap())?;
            handle_test_comparison(&atom_srv.base_url, &base_url)?;
            handle_test_comparison(&atom_srv.language, &"en".to_string())?;
            handle_test_comparison(&atom_srv.workspace.collections.len(), &(16usize))?;
            handle_test_comparison(&atom_srv.links.len(), &(2usize))?;
            handle_test_comparison(&atom_srv.links[0].rel, &"self".to_string())?;
            handle_test_comparison(&atom_srv.links[0].href, &base_url)?;
            handle_test_comparison(&atom_srv.links[1].rel, &"latest-version".to_string())?;
            handle_test_comparison(&atom_srv.links[1].href, &base_url)
        }
        Err(err) => Err(format!("XML test data was not in UTF8 format: {err}")),
    }
}

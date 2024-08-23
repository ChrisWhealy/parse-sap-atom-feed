use super::link::AtomLink;
use crate::{
    deserializers::{default_sap_content_version, edm_string::to_bool},
    xml::{default_false, default_true},
};
use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:collection>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomCollection {
    #[serde(
        rename = "@creatable",
        deserialize_with = "to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(
        rename = "@updatable",
        deserialize_with = "to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,

    #[serde(
        rename = "@deletable",
        deserialize_with = "to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,

    #[serde(
        rename = "@pageable",
        deserialize_with = "to_bool",
        default = "default_true"
    )]
    pub is_pageable: bool,

    #[serde(
        rename = "@searchable",
        deserialize_with = "to_bool",
        default = "default_false"
    )]
    pub is_searchable: bool,

    #[serde(rename = "@content-version", default = "default_sap_content_version")]
    pub content_version: String,
    #[serde(rename = "@href")]
    pub href: String,
    pub title: String,
    #[serde(rename = "member-title")]
    pub member_title: String,
    pub link: Option<AtomLink>,
}

impl std::str::FromStr for AtomCollection {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

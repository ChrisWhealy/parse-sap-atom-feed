use crate::deserializers::{
    de_str_to_bool, default_false, default_sap_content_version, default_true,
};
use crate::xml::{
    default_xml_namespace_app, default_xml_namespace_atom, default_xml_namespace_m,
    default_xml_namespace_sap, default_xml_language
};
use serde::{Deserialize, Serialize};

pub mod feed;

// use feed::Feed;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:link>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomLink {
    // Appears in the XML as the `link` attribute `xmlns:atom`
    #[serde(rename = "@atom", default = "default_xml_namespace_atom")]
    pub xml_namespace_atom: Option<String>,

    #[serde(rename = "@type")]
    pub mime_type: Option<String>,

    #[serde(rename = "@rel")]
    pub rel: String,

    #[serde(rename = "@href")]
    pub href: String,

    #[serde(rename = "@title")]
    pub title: Option<String>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:collection>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomCollection {
    #[serde(
        rename = "@creatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_creatable: bool,

    #[serde(
        rename = "@updatable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_updatable: bool,

    #[serde(
        rename = "@deletable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_deletable: bool,

    #[serde(
        rename = "@pageable",
        deserialize_with = "de_str_to_bool",
        default = "default_true"
    )]
    pub is_pageable: bool,

    #[serde(
        rename = "@searchable",
        deserialize_with = "de_str_to_bool",
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an `<atom:workspace>` tag
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomWorkspace {
    pub title: String,

    #[serde(rename = "collection")]
    pub collections: Vec<AtomCollection>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Represents an Atom service document `<app:service>`
///
/// ***WARNING:***<br>`quick-xml` strips namespace identifiers from XML tag names, and from certain attribute names!
///
/// Tag names such as `<app:service>` and `<atom:title>` will appear simply as `<service>` and `<title>`.
///
/// Attribute names prefixed with `xml` such as `xml:lang` or `xml:base` will be modified to `lang` and `base`, but
/// `xmlns:app` or `xmlns:atom` will appear without modification
#[derive(Debug, Serialize, Deserialize)]
pub struct AtomService {
    #[serde(rename = "@xmlns:app", default = "default_xml_namespace_app")]
    pub namespace_app: String,
    #[serde(rename = "@xmlns:atom", default = "default_xml_namespace_atom")]
    pub namespace_atom: Option<String>,
    #[serde(rename = "@xmlns:m", default = "default_xml_namespace_m")]
    pub namespace_m: String,
    #[serde(rename = "@xmlns:sap", default = "default_xml_namespace_sap")]
    pub namespace_sap: String,
    #[serde(rename = "@lang", default = "default_xml_language")]
    pub language: String,
    #[serde(rename = "@base")]
    pub base_url: String,
    pub workspace: AtomWorkspace,
    #[serde(rename = "link")]
    pub links: Vec<AtomLink>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;

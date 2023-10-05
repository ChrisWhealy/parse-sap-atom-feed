# Parse an SAP OData V2 XML `atom:Feed`

***This is a work in progress!***

The crate is designed to work in conjunction with the source code generated from the crate [`parse-sap-odata`](https://crates.io/crates/parse-sap-odata).

Crate `parse-sap-odata` is invoked by a Rust build script to parse an OData XML metadata file and generates the Rust `struct`s and `enum`s needed to consume the entity set data described by that metadata.

This crate `parse-sap-atom-feed` then makes use of the `struct`s and `enum`s generated above and allows your business app to consume the XML returned when requesting entity sets from this OData service.

## Changes in V0.2

The main change in version 0.2 is to adopt `quick-xml`'s [`@`-prefixed attribute syntax](https://docs.rs/quick-xml/latest/quick_xml/de/index.html#mapping-xml-to-rust-types) introduced in `0.27` and higher.

## Example Usage

You want to develop a Rust application that can consume the entity set data exposed by an SAP OData V2 service.

For the purposes of instruction, let's say you're working with a custom OData service that displays services ordered either by functional location or by date:

| | |
|---|---|
| Rust Application | `zcustom-service-orders`
| OData Service URL | `https://my-sap-server.my-domain.com/sap/opu/odata/sap`
| OData Service Name | `ZCUSTOM_SERVICE_ORDERS_SRV`
| OData Schema Name | `ZCUSTOM_SERVICE_ORDERS`
| Entity Names | `SERVICE_ORDERS_BY_FLOC`<br>`SERVICE_ORDERS_BY_DATE`
| Entity Type | `ZServiceOrder`

The general approach to consuming such a service is as follows:

1. Create a new Rust binary application `cargo new zcustom-service-orders`
1. `cd zcustom-service-orders`
1. Edit `Cargo.toml` to include at least the following dependencies
   ```rust
   [build-dependencies]
   parse-sap-odata = "1.1"

   [dependencies]
   chrono = { version = "0.4", features = ["serde"]}
   parse-sap-atom-feed = "0.1"
   rust_decimal = "1.30"
   serde = { version = "1.0", features = ["derive"] }
   uuid = { version = "1.4", features = ["serde"]}
   ```
1. `mkdir odata`
1. Display your service's metadata.
E.G. `https://my-sap-server.my-domain.com/sap/opu/odata/sap/ZCUSTOM_SERVICE_ORDERS_SRV/$metadata`
1. Save the metadata as the file `zcustom_service_orders.xml` in the `odata` directory
1. Create a `build.rs` file in same directory as `Cargo.toml` and add at least the following:
   ```rust
   use parse_sap_odata::parser::gen_src;

   fn main() {
       gen_src("zcustom_service_orders", "ZCUSTOM_SERVICE_ORDERS");
   }
   ```
1. In `src/main.rs` include at least:
   ```rust
   use parse_sap_atom_feed::{
       Feed,
       xml::sanitise_xml,
       str::{self, FromStr},
   };

   include!(concat!(env!("OUT_DIR"), "/zcustom_service_orders.rs"));

   use zcustom_service_orders::*;

   fn main() {
      let raw_xml: String = /* Whatever code is needed to fetch the entity set data as a raw XML string */;

      // You might need to sanitise the raw XML string before attempting to parse it
      let clean_xml = sanitise_xml(raw_xml);

      let srv_orders_by_floc = Feed::<ZServiceOrder>::from_str(&clean_xml);
   }
   ```

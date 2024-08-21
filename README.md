# Parse an XML `atom:Feed` received from an SAP V2 OData Service

This crate is designed to work in conjunction with the source code generated by the [`parse-sap-odata`](https://crates.io/crates/parse-sap-odata) crate.

## Use `parse-sap-odata` at Build Time

The functionality in crate `parse-sap-odata` is invoked by a Rust build script in the business application that interacts with an SAP V2 OData service.  The build script generates a pair of Rust modules (one for the service document, and the other for the metadata document) containing all the `struct`s and `enum`s needed to interact with the OData service at runtime.

## Use `parse-sap-atom-feed` at Runtime

This crate then uses the modules generated above to consume the `atom:Feed` XML returned when interacting with the OData service.

## Table of Contents

* [Usage](./docs/usage.md)
* [Parsing Decimal Values](./docs/decimals.md)

## Change Log

| Version | Task    | Description
|--:|---------|---
1.1.0 | Feature | Custom parser for `rust_decimal::Decimal` values
1.0.1 | Fix     | Remove redundant code
1.0.0 | Release | Promote to version 1.0
0.2.9 | Chore   | Handle all test failures without panic
0.2.8 | Feature | Implement `std::str::FromStr` for Atom types
0.2.7 | Feature | Parse an Atom feed of `<entry>` elements
0.2.6 | Feature | As per <https://validator.w3.org/feed/docs/atom.html#requiredEntryElements>, the Atom `<content>` element should either contain or link to, the complete content of the entry.<br>If the `src` attribute is present, then the `<properties>` element (if present) exists as a sibling of the `<content>` element.<br>If the `src` attribute is missing, the `<properties>` element must exist as a child of the `<content>` element.<br><br>Add support for out-of-order XML elements (quick-xml feature `overlapped-lists`)
0.2.5 | Feature | Read generic OData service document
0.2.4 | Fix     | Update `Cargo.toml` dependency versions

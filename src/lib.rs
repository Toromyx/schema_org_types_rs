//! This crate provides automatically generated Rust types of [Schema.org](https://schema.org) schemas.
//!
//! ## Features
//!
//! - `all-schema-sections` — include all schema sections
//!   - enables *general-schema-section*, *auto-schema-section*, *bib-schema-section*, *health-lifesci-schema-section*, *pending-schema-section*, *attic-schema-section*, *meta-schema-section*
//! - `general-schema-section` — include all schemas without a special section
//!   - enabled by *all-schema-sections*
//! - `auto-schema-section` — include <https://schema.org/docs/auto.home.html>
//!   - enabled by *all-schema-sections*
//! - `bib-schema-section` — include <https://schema.org/docs/bib.home.html>
//!   - enabled by *all-schema-sections*
//! - `health-lifesci-schema-section` — include <https://schema.org/docs/health-lifesci.home.html>
//!   - enabled by *all-schema-sections*
//! - `pending-schema-section` — include <https://schema.org/docs/pending.home.html>
//!   - enabled by *all-schema-sections*
//! - `attic-schema-section` — include <https://schema.org/docs/attic.home.html>
//!   - enabled by *all-schema-sections*
//! - `meta-schema-section` — include <https://schema.org/docs/meta.home.html>
//!   - enabled by *all-schema-sections*
//! - `derive-all` — add all derives on the schemas
//!   - enables *derive-debug*, *derive-clone*
//! - `derive-debug` — derive [`Debug`] for the schemas; this needs a `#![recursion_limit = "512"]` in your crate
//!   - enabled by *derive-all*
//! - `derive-clone` — derive [`Clone`] for the schemas
//!   - enabled by *derive-all*
//! - `fallible` — add a last `*Fail`-variant to all properties which should capture all failed deserialization attempts with their value, see [`fallible`]
//!
//! ### Optional Dependencies
//!
//! - `serde` — enable serialization and deserialization via [serde](https://serde.rs/)
//!   - enables *dep:serde*, *dep:serde_with*, *json-number/serde*, *url/serde*
//!

// This recursion limit is necessary for the [`Debug`] derive macro.
#![recursion_limit = "512"]
// https://doc.rust-lang.org/rustdoc/unstable-features.html#doc_auto_cfg-automatically-generate-doccfg
#![cfg_attr(doc, feature(doc_auto_cfg))]

pub mod date_types;
#[cfg(any(feature = "fallible", doc))]
mod fallible;
pub mod number_types;
mod schemas;

pub use json_number;
pub use schemas::*;
pub use speedate;
pub use url;

#[cfg(test)]
mod tests;

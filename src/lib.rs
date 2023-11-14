//! This crate provides automatically generated Rust types of [Schema.org](https://schema.org) schemas.
//!
//! You will need a `#![recursion_limit = "512"]` when compiling this crate.
//!
//! ## Features
//!
//! - `derive-all` — add all derives on the schemas
//!   - enables *derive-debug*, *derive-clone*
//! - `derive-debug` — derive [`Debug`] for the schemas
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

#![recursion_limit = "512"]
// https://doc.rust-lang.org/rustdoc/unstable-features.html#doc_auto_cfg-automatically-generate-doccfg
#![cfg_attr(doc, feature(doc_auto_cfg))]

pub mod date_types;
#[cfg(any(feature = "fallible", doc))]
pub mod fallible;
pub mod number_types;
mod schemas;

pub use json_number;
pub use schemas::*;
pub use speedate;
pub use url;

#[cfg(test)]
mod tests;

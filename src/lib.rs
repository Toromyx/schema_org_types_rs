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
//! - `derive-debug` — derive [`Debug`] for the schemas
//!   - enabled by *derive-all*
//! - `derive-clone` — derive [`Clone`] for the schemas
//!   - enabled by *derive-all*
//!
//! ### Optional Dependencies
//!
//! - `serde` — enable serialization and deserialization via [serde](https://serde.rs/)
//!   - enables *dep:serde*, *dep:serde_with*
//!
//! ### Generated Features
//!
//! Each schema has its own feature. E.g. <https://schema.org/Recipe> has the feature `recipe-schema`. Property schema features have the `-property-schema` suffix to avoid duplicate feature names.
//!

// This recursion limit is necessary for the [`Debug`] derive macro.
#![cfg_attr(feature = "derive-debug", recursion_limit = "512")]

pub mod date_types;
mod schemas;

pub use schemas::*;
pub use speedate;

#[cfg(test)]
mod tests {
    use schema_org_types::*;
    use serde_json::{json, Value};

    #[test]
    fn test_date_serde() {
        let date_strings = vec![json!("2023-09-16")];
        let dates: Vec<Date> = date_strings
            .iter()
            .map(|value| serde_json::from_value(value.clone()).unwrap())
            .collect();
        let date_strings_serde: Vec<Value> = dates
            .iter()
            .map(|date| serde_json::to_value(date).unwrap())
            .collect();
        assert_eq!(date_strings, date_strings_serde);
    }

    #[test]
    fn test_time_serde() {
        let time_strings = vec![
            json!("20:07:37Z"),
            json!("20:07:37-02:00"),
            json!("20:07:37"),
        ];
        let times: Vec<Time> = time_strings
            .iter()
            .map(|value| serde_json::from_value(value.clone()).unwrap())
            .collect();
        let time_strings_serde: Vec<Value> = times
            .iter()
            .map(|time| serde_json::to_value(time).unwrap())
            .collect();
        assert_eq!(time_strings, time_strings_serde);
    }

    #[test]
    fn test_date_time_serde() {
        let date_time_strings = vec![
            json!("2023-09-16T20:07:37Z"),
            json!("2023-09-16T20:07:37-02:00"),
            json!("2023-09-16T20:07:37"),
        ];
        let date_times: Vec<DateTime> = date_time_strings
            .iter()
            .map(|value| serde_json::from_value(value.clone()).unwrap())
            .collect();
        let date_time_strings_serde: Vec<Value> = date_times
            .iter()
            .map(|date_time| serde_json::to_value(date_time).unwrap())
            .collect();
        assert_eq!(date_time_strings, date_time_strings_serde);
    }

    #[test]
    fn test_enumeration_serialization() {
        let string = serde_json::to_string(&ActionStatusType::ActiveActionStatus).unwrap();
        assert_eq!(string, "\"ActiveActionStatus\"".to_string());
    }
}

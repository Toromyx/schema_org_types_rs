//! This module includes code used in fallible deserialization.

use std::collections::HashMap;

/// This enumerations represents an arbitrary failed deserialization.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FailValue {
	Bool(bool),
	U64(u64),
	U128(u128),
	I64(i64),
	I128(i128),
	F64(f64),
	String(String),
	Map(HashMap<String, FailValue>),
}

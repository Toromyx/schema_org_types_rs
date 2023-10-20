use std::{fmt, fmt::Formatter};

use json_number::{Buffer, NumberBuf};
use serde::{
	de::{Error, Visitor},
	Deserialize, Deserializer, Serialize, Serializer,
};

use crate::number_types::Number;

impl Serialize for Number {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		if self.0.has_decimal_point() {
			serializer.serialize_f64(self.0.as_f64_lossy())
		} else if let Some(v) = self.0.as_i64() {
			serializer.serialize_i64(v)
		} else if let Some(v) = self.0.as_u64() {
			serializer.serialize_u64(v)
		} else {
			serializer.serialize_str(self.0.as_str())
		}
	}
}

struct NumberVisitor;

impl<'de> Visitor<'de> for NumberVisitor {
	type Value = Number;

	fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
		formatter.write_str("JSON number or string")
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: Error,
	{
		let number_buf = NumberBuf::from(v);
		Ok(Number(number_buf))
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: Error,
	{
		let number_buf = NumberBuf::from(v);
		Ok(Number(number_buf))
	}

	fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
	where
		E: Error,
	{
		let number_buf = NumberBuf::try_from(v)
			.map_err(|_| E::invalid_value(serde::de::Unexpected::Float(v), &self))?;
		Ok(Number(number_buf))
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: Error,
	{
		let number_buf = NumberBuf::new(Vec::from_bytes(v.as_bytes()))
			.map_err(|_| E::invalid_value(serde::de::Unexpected::Str(v), &self))?;
		Ok(Number(number_buf))
	}
}

impl<'de> Deserialize<'de> for Number {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(NumberVisitor)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! serde_tests {
		{ $($id:ident: $input:literal => $output:literal),* } => {
			$(
				#[test]
				fn $id () {
					let number: Number = ::serde_json::from_value(::serde_json::json!($input)).unwrap();
					assert_eq!(::serde_json::to_value(&number).unwrap(), ::serde_json::json!($output))
				}
			)*
		};
	}

	serde_tests! {
		serde_one_int: 1 => 1,
		serde_one_str: "1" => 1,
		serde_minus_one_int: -1 => -1,
		serde_minus_one_str: "-1" => -1,
		serde_u64_max_int: 18446744073709551615u64 => 18_446_744_073_709_551_615u64,
		serde_u64_max_str: "18446744073709551615" => 18_446_744_073_709_551_615u64,
		serde_u64_max_plus_one: "18446744073709551616" => "18446744073709551616"
	}
}

/// <https://schema.org/MeasurementMethodEnum>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MeasurementMethodEnum {
	/// <https://schema.org/ExampleMeasurementMethodEnum>
	ExampleMeasurementMethodEnum,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MeasurementMethodEnum {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MeasurementMethodEnum::ExampleMeasurementMethodEnum => serializer
					.serialize_unit_variant(
						"MeasurementMethodEnum",
						0u32,
						"ExampleMeasurementMethodEnum",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for MeasurementMethodEnum {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ExampleMeasurementMethodEnum,
			}
			struct FieldVisitor;
			impl<'de> de::Visitor<'de> for FieldVisitor {
				type Value = Field;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("variant identifier")
				}
				fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						"ExampleMeasurementMethodEnum" => Ok(Field::ExampleMeasurementMethodEnum),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ExampleMeasurementMethodEnum" => Ok(Field::ExampleMeasurementMethodEnum),
						_ => {
							let value = &String::from_utf8_lossy(value);
							Err(de::Error::unknown_variant(value, VARIANTS))
						}
					}
				}
			}
			impl<'de> Deserialize<'de> for Field {
				fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where
					D: Deserializer<'de>,
				{
					deserializer.deserialize_identifier(FieldVisitor)
				}
			}
			struct EnumerationVisitor;
			impl<'de> Visitor<'de> for EnumerationVisitor {
				type Value = MeasurementMethodEnum;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MeasurementMethodEnum")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ExampleMeasurementMethodEnum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MeasurementMethodEnum::ExampleMeasurementMethodEnum)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["ExampleMeasurementMethodEnum"];
			deserializer.deserialize_enum("MeasurementMethodEnum", VARIANTS, EnumerationVisitor)
		}
	}
}

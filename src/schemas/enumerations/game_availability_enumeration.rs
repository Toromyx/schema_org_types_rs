/// <https://schema.org/GameAvailabilityEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GameAvailabilityEnumeration {
	/// <https://schema.org/DemoGameAvailability>
	DemoGameAvailability,
	/// <https://schema.org/FullGameAvailability>
	FullGameAvailability,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for GameAvailabilityEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				GameAvailabilityEnumeration::DemoGameAvailability => serializer
					.serialize_unit_variant(
						"GameAvailabilityEnumeration",
						0u32,
						"DemoGameAvailability",
					),
				GameAvailabilityEnumeration::FullGameAvailability => serializer
					.serialize_unit_variant(
						"GameAvailabilityEnumeration",
						1u32,
						"FullGameAvailability",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for GameAvailabilityEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				DemoGameAvailability,
				FullGameAvailability,
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
						"DemoGameAvailability" => Ok(Field::DemoGameAvailability),
						"FullGameAvailability" => Ok(Field::FullGameAvailability),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"DemoGameAvailability" => Ok(Field::DemoGameAvailability),
						b"FullGameAvailability" => Ok(Field::FullGameAvailability),
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
				type Value = GameAvailabilityEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema GameAvailabilityEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::DemoGameAvailability, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GameAvailabilityEnumeration::DemoGameAvailability)
						}
						(Field::FullGameAvailability, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GameAvailabilityEnumeration::FullGameAvailability)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["DemoGameAvailability", "FullGameAvailability"];
			deserializer.deserialize_enum(
				"GameAvailabilityEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

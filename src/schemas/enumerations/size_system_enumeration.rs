/// <https://schema.org/SizeSystemEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum SizeSystemEnumeration {
	/// <https://schema.org/SizeSystemImperial>
	SizeSystemImperial,
	/// <https://schema.org/SizeSystemMetric>
	SizeSystemMetric,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for SizeSystemEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				SizeSystemEnumeration::SizeSystemImperial => serializer.serialize_unit_variant(
					"SizeSystemEnumeration",
					0u32,
					"SizeSystemImperial",
				),
				SizeSystemEnumeration::SizeSystemMetric => serializer.serialize_unit_variant(
					"SizeSystemEnumeration",
					1u32,
					"SizeSystemMetric",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for SizeSystemEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				SizeSystemImperial,
				SizeSystemMetric,
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
						"SizeSystemImperial" => Ok(Field::SizeSystemImperial),
						"SizeSystemMetric" => Ok(Field::SizeSystemMetric),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"SizeSystemImperial" => Ok(Field::SizeSystemImperial),
						b"SizeSystemMetric" => Ok(Field::SizeSystemMetric),
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
				type Value = SizeSystemEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema SizeSystemEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::SizeSystemImperial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(SizeSystemEnumeration::SizeSystemImperial)
						}
						(Field::SizeSystemMetric, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(SizeSystemEnumeration::SizeSystemMetric)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["SizeSystemImperial", "SizeSystemMetric"];
			deserializer.deserialize_enum("SizeSystemEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/GenderType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GenderType {
	/// <https://schema.org/Female>
	Female,
	/// <https://schema.org/Male>
	Male,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for GenderType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				GenderType::Female => {
					serializer.serialize_unit_variant("GenderType", 0u32, "Female")
				}
				GenderType::Male => serializer.serialize_unit_variant("GenderType", 1u32, "Male"),
			}
		}
	}
	impl<'de> Deserialize<'de> for GenderType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Female,
				Male,
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
						"Female" => Ok(Field::Female),
						"Male" => Ok(Field::Male),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Female" => Ok(Field::Female),
						b"Male" => Ok(Field::Male),
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
				type Value = GenderType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema GenderType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Female, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GenderType::Female)
						}
						(Field::Male, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GenderType::Male)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["Female", "Male"];
			deserializer.deserialize_enum("GenderType", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/LegalValueLevel>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum LegalValueLevel {
	/// <https://schema.org/AuthoritativeLegalValue>
	AuthoritativeLegalValue,
	/// <https://schema.org/DefinitiveLegalValue>
	DefinitiveLegalValue,
	/// <https://schema.org/OfficialLegalValue>
	OfficialLegalValue,
	/// <https://schema.org/UnofficialLegalValue>
	UnofficialLegalValue,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for LegalValueLevel {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				LegalValueLevel::AuthoritativeLegalValue => serializer.serialize_unit_variant(
					"LegalValueLevel",
					0u32,
					"AuthoritativeLegalValue",
				),
				LegalValueLevel::DefinitiveLegalValue => serializer.serialize_unit_variant(
					"LegalValueLevel",
					1u32,
					"DefinitiveLegalValue",
				),
				LegalValueLevel::OfficialLegalValue => {
					serializer.serialize_unit_variant("LegalValueLevel", 2u32, "OfficialLegalValue")
				}
				LegalValueLevel::UnofficialLegalValue => serializer.serialize_unit_variant(
					"LegalValueLevel",
					3u32,
					"UnofficialLegalValue",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for LegalValueLevel {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AuthoritativeLegalValue,
				DefinitiveLegalValue,
				OfficialLegalValue,
				UnofficialLegalValue,
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
						"AuthoritativeLegalValue" => Ok(Field::AuthoritativeLegalValue),
						"DefinitiveLegalValue" => Ok(Field::DefinitiveLegalValue),
						"OfficialLegalValue" => Ok(Field::OfficialLegalValue),
						"UnofficialLegalValue" => Ok(Field::UnofficialLegalValue),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AuthoritativeLegalValue" => Ok(Field::AuthoritativeLegalValue),
						b"DefinitiveLegalValue" => Ok(Field::DefinitiveLegalValue),
						b"OfficialLegalValue" => Ok(Field::OfficialLegalValue),
						b"UnofficialLegalValue" => Ok(Field::UnofficialLegalValue),
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
				type Value = LegalValueLevel;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema LegalValueLevel")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AuthoritativeLegalValue, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(LegalValueLevel::AuthoritativeLegalValue)
						}
						(Field::DefinitiveLegalValue, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(LegalValueLevel::DefinitiveLegalValue)
						}
						(Field::OfficialLegalValue, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(LegalValueLevel::OfficialLegalValue)
						}
						(Field::UnofficialLegalValue, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(LegalValueLevel::UnofficialLegalValue)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AuthoritativeLegalValue",
				"DefinitiveLegalValue",
				"OfficialLegalValue",
				"UnofficialLegalValue",
			];
			deserializer.deserialize_enum("LegalValueLevel", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/ContactPointOption>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ContactPointOption {
	/// <https://schema.org/HearingImpairedSupported>
	HearingImpairedSupported,
	/// <https://schema.org/TollFree>
	TollFree,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ContactPointOption {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ContactPointOption::HearingImpairedSupported => serializer.serialize_unit_variant(
					"ContactPointOption",
					0u32,
					"HearingImpairedSupported",
				),
				ContactPointOption::TollFree => {
					serializer.serialize_unit_variant("ContactPointOption", 1u32, "TollFree")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for ContactPointOption {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				HearingImpairedSupported,
				TollFree,
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
						"HearingImpairedSupported" => Ok(Field::HearingImpairedSupported),
						"TollFree" => Ok(Field::TollFree),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"HearingImpairedSupported" => Ok(Field::HearingImpairedSupported),
						b"TollFree" => Ok(Field::TollFree),
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
				type Value = ContactPointOption;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ContactPointOption")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::HearingImpairedSupported, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ContactPointOption::HearingImpairedSupported)
						}
						(Field::TollFree, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ContactPointOption::TollFree)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["HearingImpairedSupported", "TollFree"];
			deserializer.deserialize_enum("ContactPointOption", VARIANTS, EnumerationVisitor)
		}
	}
}

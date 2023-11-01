/// <https://schema.org/RsvpResponseType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum RsvpResponseType {
	/// <https://schema.org/RsvpResponseMaybe>
	RsvpResponseMaybe,
	/// <https://schema.org/RsvpResponseNo>
	RsvpResponseNo,
	/// <https://schema.org/RsvpResponseYes>
	RsvpResponseYes,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for RsvpResponseType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				RsvpResponseType::RsvpResponseMaybe => {
					serializer.serialize_unit_variant("RsvpResponseType", 0u32, "RsvpResponseMaybe")
				}
				RsvpResponseType::RsvpResponseNo => {
					serializer.serialize_unit_variant("RsvpResponseType", 1u32, "RsvpResponseNo")
				}
				RsvpResponseType::RsvpResponseYes => {
					serializer.serialize_unit_variant("RsvpResponseType", 2u32, "RsvpResponseYes")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for RsvpResponseType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				RsvpResponseMaybe,
				RsvpResponseNo,
				RsvpResponseYes,
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
						"RsvpResponseMaybe" => Ok(Field::RsvpResponseMaybe),
						"RsvpResponseNo" => Ok(Field::RsvpResponseNo),
						"RsvpResponseYes" => Ok(Field::RsvpResponseYes),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"RsvpResponseMaybe" => Ok(Field::RsvpResponseMaybe),
						b"RsvpResponseNo" => Ok(Field::RsvpResponseNo),
						b"RsvpResponseYes" => Ok(Field::RsvpResponseYes),
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
				type Value = RsvpResponseType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema RsvpResponseType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::RsvpResponseMaybe, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RsvpResponseType::RsvpResponseMaybe)
						}
						(Field::RsvpResponseNo, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RsvpResponseType::RsvpResponseNo)
						}
						(Field::RsvpResponseYes, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RsvpResponseType::RsvpResponseYes)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["RsvpResponseMaybe", "RsvpResponseNo", "RsvpResponseYes"];
			deserializer.deserialize_enum("RsvpResponseType", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/LegalForceStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum LegalForceStatus {
	/// <https://schema.org/InForce>
	InForce,
	/// <https://schema.org/NotInForce>
	NotInForce,
	/// <https://schema.org/PartiallyInForce>
	PartiallyInForce,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for LegalForceStatus {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				LegalForceStatus::InForce => {
					serializer.serialize_unit_variant("LegalForceStatus", 0u32, "InForce")
				}
				LegalForceStatus::NotInForce => {
					serializer.serialize_unit_variant("LegalForceStatus", 1u32, "NotInForce")
				}
				LegalForceStatus::PartiallyInForce => {
					serializer.serialize_unit_variant("LegalForceStatus", 2u32, "PartiallyInForce")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for LegalForceStatus {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				InForce,
				NotInForce,
				PartiallyInForce,
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
						"InForce" => Ok(Field::InForce),
						"NotInForce" => Ok(Field::NotInForce),
						"PartiallyInForce" => Ok(Field::PartiallyInForce),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"InForce" => Ok(Field::InForce),
						b"NotInForce" => Ok(Field::NotInForce),
						b"PartiallyInForce" => Ok(Field::PartiallyInForce),
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
				type Value = LegalForceStatus;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema LegalForceStatus")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::InForce, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(LegalForceStatus::InForce)
						}
						(Field::NotInForce, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(LegalForceStatus::NotInForce)
						}
						(Field::PartiallyInForce, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(LegalForceStatus::PartiallyInForce)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["InForce", "NotInForce", "PartiallyInForce"];
			deserializer.deserialize_enum("LegalForceStatus", VARIANTS, EnumerationVisitor)
		}
	}
}

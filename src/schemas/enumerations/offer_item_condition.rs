/// <https://schema.org/OfferItemCondition>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum OfferItemCondition {
	/// <https://schema.org/DamagedCondition>
	DamagedCondition,
	/// <https://schema.org/NewCondition>
	NewCondition,
	/// <https://schema.org/RefurbishedCondition>
	RefurbishedCondition,
	/// <https://schema.org/UsedCondition>
	UsedCondition,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for OfferItemCondition {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				OfferItemCondition::DamagedCondition => serializer.serialize_unit_variant(
					"OfferItemCondition",
					0u32,
					"DamagedCondition",
				),
				OfferItemCondition::NewCondition => {
					serializer.serialize_unit_variant("OfferItemCondition", 1u32, "NewCondition")
				}
				OfferItemCondition::RefurbishedCondition => serializer.serialize_unit_variant(
					"OfferItemCondition",
					2u32,
					"RefurbishedCondition",
				),
				OfferItemCondition::UsedCondition => {
					serializer.serialize_unit_variant("OfferItemCondition", 3u32, "UsedCondition")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for OfferItemCondition {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				DamagedCondition,
				NewCondition,
				RefurbishedCondition,
				UsedCondition,
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
						"DamagedCondition" => Ok(Field::DamagedCondition),
						"NewCondition" => Ok(Field::NewCondition),
						"RefurbishedCondition" => Ok(Field::RefurbishedCondition),
						"UsedCondition" => Ok(Field::UsedCondition),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"DamagedCondition" => Ok(Field::DamagedCondition),
						b"NewCondition" => Ok(Field::NewCondition),
						b"RefurbishedCondition" => Ok(Field::RefurbishedCondition),
						b"UsedCondition" => Ok(Field::UsedCondition),
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
				type Value = OfferItemCondition;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema OfferItemCondition")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::DamagedCondition, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OfferItemCondition::DamagedCondition)
						}
						(Field::NewCondition, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OfferItemCondition::NewCondition)
						}
						(Field::RefurbishedCondition, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OfferItemCondition::RefurbishedCondition)
						}
						(Field::UsedCondition, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OfferItemCondition::UsedCondition)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"DamagedCondition",
				"NewCondition",
				"RefurbishedCondition",
				"UsedCondition",
			];
			deserializer.deserialize_enum("OfferItemCondition", VARIANTS, EnumerationVisitor)
		}
	}
}

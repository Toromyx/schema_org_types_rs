/// <https://schema.org/NLNonprofitType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum NlNonprofitType {
	/// <https://schema.org/NonprofitANBI>
	NonprofitAnbi,
	/// <https://schema.org/NonprofitSBBI>
	NonprofitSbbi,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for NlNonprofitType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				NlNonprofitType::NonprofitAnbi => {
					serializer.serialize_unit_variant("NlNonprofitType", 0u32, "NonprofitAnbi")
				}
				NlNonprofitType::NonprofitSbbi => {
					serializer.serialize_unit_variant("NlNonprofitType", 1u32, "NonprofitSbbi")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for NlNonprofitType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				NonprofitAnbi,
				NonprofitSbbi,
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
						"NonprofitAnbi" => Ok(Field::NonprofitAnbi),
						"NonprofitSbbi" => Ok(Field::NonprofitSbbi),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"NonprofitAnbi" => Ok(Field::NonprofitAnbi),
						b"NonprofitSbbi" => Ok(Field::NonprofitSbbi),
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
				type Value = NlNonprofitType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema NLNonprofitType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::NonprofitAnbi, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(NlNonprofitType::NonprofitAnbi)
						}
						(Field::NonprofitSbbi, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(NlNonprofitType::NonprofitSbbi)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["NonprofitAnbi", "NonprofitSbbi"];
			deserializer.deserialize_enum("NlNonprofitType", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/BoardingPolicyType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum BoardingPolicyType {
	/// <https://schema.org/GroupBoardingPolicy>
	GroupBoardingPolicy,
	/// <https://schema.org/ZoneBoardingPolicy>
	ZoneBoardingPolicy,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for BoardingPolicyType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				BoardingPolicyType::GroupBoardingPolicy => serializer.serialize_unit_variant(
					"BoardingPolicyType",
					0u32,
					"GroupBoardingPolicy",
				),
				BoardingPolicyType::ZoneBoardingPolicy => serializer.serialize_unit_variant(
					"BoardingPolicyType",
					1u32,
					"ZoneBoardingPolicy",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for BoardingPolicyType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				GroupBoardingPolicy,
				ZoneBoardingPolicy,
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
						"GroupBoardingPolicy" => Ok(Field::GroupBoardingPolicy),
						"ZoneBoardingPolicy" => Ok(Field::ZoneBoardingPolicy),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"GroupBoardingPolicy" => Ok(Field::GroupBoardingPolicy),
						b"ZoneBoardingPolicy" => Ok(Field::ZoneBoardingPolicy),
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
				type Value = BoardingPolicyType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema BoardingPolicyType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::GroupBoardingPolicy, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BoardingPolicyType::GroupBoardingPolicy)
						}
						(Field::ZoneBoardingPolicy, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BoardingPolicyType::ZoneBoardingPolicy)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["GroupBoardingPolicy", "ZoneBoardingPolicy"];
			deserializer.deserialize_enum("BoardingPolicyType", VARIANTS, EnumerationVisitor)
		}
	}
}

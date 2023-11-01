/// <https://schema.org/SteeringPositionValue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum SteeringPositionValue {
	/// <https://schema.org/LeftHandDriving>
	LeftHandDriving,
	/// <https://schema.org/RightHandDriving>
	RightHandDriving,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for SteeringPositionValue {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				SteeringPositionValue::LeftHandDriving => serializer.serialize_unit_variant(
					"SteeringPositionValue",
					0u32,
					"LeftHandDriving",
				),
				SteeringPositionValue::RightHandDriving => serializer.serialize_unit_variant(
					"SteeringPositionValue",
					1u32,
					"RightHandDriving",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for SteeringPositionValue {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				LeftHandDriving,
				RightHandDriving,
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
						"LeftHandDriving" => Ok(Field::LeftHandDriving),
						"RightHandDriving" => Ok(Field::RightHandDriving),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"LeftHandDriving" => Ok(Field::LeftHandDriving),
						b"RightHandDriving" => Ok(Field::RightHandDriving),
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
				type Value = SteeringPositionValue;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema SteeringPositionValue")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::LeftHandDriving, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(SteeringPositionValue::LeftHandDriving)
						}
						(Field::RightHandDriving, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(SteeringPositionValue::RightHandDriving)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["LeftHandDriving", "RightHandDriving"];
			deserializer.deserialize_enum("SteeringPositionValue", VARIANTS, EnumerationVisitor)
		}
	}
}

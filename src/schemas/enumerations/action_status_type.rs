/// <https://schema.org/ActionStatusType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ActionStatusType {
	/// <https://schema.org/ActiveActionStatus>
	ActiveActionStatus,
	/// <https://schema.org/CompletedActionStatus>
	CompletedActionStatus,
	/// <https://schema.org/FailedActionStatus>
	FailedActionStatus,
	/// <https://schema.org/PotentialActionStatus>
	PotentialActionStatus,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ActionStatusType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ActionStatusType::ActiveActionStatus => serializer.serialize_unit_variant(
					"ActionStatusType",
					0u32,
					"ActiveActionStatus",
				),
				ActionStatusType::CompletedActionStatus => serializer.serialize_unit_variant(
					"ActionStatusType",
					1u32,
					"CompletedActionStatus",
				),
				ActionStatusType::FailedActionStatus => serializer.serialize_unit_variant(
					"ActionStatusType",
					2u32,
					"FailedActionStatus",
				),
				ActionStatusType::PotentialActionStatus => serializer.serialize_unit_variant(
					"ActionStatusType",
					3u32,
					"PotentialActionStatus",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for ActionStatusType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ActiveActionStatus,
				CompletedActionStatus,
				FailedActionStatus,
				PotentialActionStatus,
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
						"ActiveActionStatus" => Ok(Field::ActiveActionStatus),
						"CompletedActionStatus" => Ok(Field::CompletedActionStatus),
						"FailedActionStatus" => Ok(Field::FailedActionStatus),
						"PotentialActionStatus" => Ok(Field::PotentialActionStatus),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ActiveActionStatus" => Ok(Field::ActiveActionStatus),
						b"CompletedActionStatus" => Ok(Field::CompletedActionStatus),
						b"FailedActionStatus" => Ok(Field::FailedActionStatus),
						b"PotentialActionStatus" => Ok(Field::PotentialActionStatus),
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
				type Value = ActionStatusType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ActionStatusType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ActiveActionStatus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ActionStatusType::ActiveActionStatus)
						}
						(Field::CompletedActionStatus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ActionStatusType::CompletedActionStatus)
						}
						(Field::FailedActionStatus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ActionStatusType::FailedActionStatus)
						}
						(Field::PotentialActionStatus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ActionStatusType::PotentialActionStatus)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ActiveActionStatus",
				"CompletedActionStatus",
				"FailedActionStatus",
				"PotentialActionStatus",
			];
			deserializer.deserialize_enum("ActionStatusType", VARIANTS, EnumerationVisitor)
		}
	}
}

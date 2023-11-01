/// <https://schema.org/PaymentStatusType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PaymentStatusType {
	/// <https://schema.org/PaymentAutomaticallyApplied>
	PaymentAutomaticallyApplied,
	/// <https://schema.org/PaymentComplete>
	PaymentComplete,
	/// <https://schema.org/PaymentDeclined>
	PaymentDeclined,
	/// <https://schema.org/PaymentDue>
	PaymentDue,
	/// <https://schema.org/PaymentPastDue>
	PaymentPastDue,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for PaymentStatusType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PaymentStatusType::PaymentAutomaticallyApplied => serializer
					.serialize_unit_variant(
						"PaymentStatusType",
						0u32,
						"PaymentAutomaticallyApplied",
					),
				PaymentStatusType::PaymentComplete => {
					serializer.serialize_unit_variant("PaymentStatusType", 1u32, "PaymentComplete")
				}
				PaymentStatusType::PaymentDeclined => {
					serializer.serialize_unit_variant("PaymentStatusType", 2u32, "PaymentDeclined")
				}
				PaymentStatusType::PaymentDue => {
					serializer.serialize_unit_variant("PaymentStatusType", 3u32, "PaymentDue")
				}
				PaymentStatusType::PaymentPastDue => {
					serializer.serialize_unit_variant("PaymentStatusType", 4u32, "PaymentPastDue")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for PaymentStatusType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				PaymentAutomaticallyApplied,
				PaymentComplete,
				PaymentDeclined,
				PaymentDue,
				PaymentPastDue,
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
						"PaymentAutomaticallyApplied" => Ok(Field::PaymentAutomaticallyApplied),
						"PaymentComplete" => Ok(Field::PaymentComplete),
						"PaymentDeclined" => Ok(Field::PaymentDeclined),
						"PaymentDue" => Ok(Field::PaymentDue),
						"PaymentPastDue" => Ok(Field::PaymentPastDue),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"PaymentAutomaticallyApplied" => Ok(Field::PaymentAutomaticallyApplied),
						b"PaymentComplete" => Ok(Field::PaymentComplete),
						b"PaymentDeclined" => Ok(Field::PaymentDeclined),
						b"PaymentDue" => Ok(Field::PaymentDue),
						b"PaymentPastDue" => Ok(Field::PaymentPastDue),
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
				type Value = PaymentStatusType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema PaymentStatusType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::PaymentAutomaticallyApplied, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentStatusType::PaymentAutomaticallyApplied)
						}
						(Field::PaymentComplete, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentStatusType::PaymentComplete)
						}
						(Field::PaymentDeclined, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentStatusType::PaymentDeclined)
						}
						(Field::PaymentDue, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentStatusType::PaymentDue)
						}
						(Field::PaymentPastDue, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PaymentStatusType::PaymentPastDue)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"PaymentAutomaticallyApplied",
				"PaymentComplete",
				"PaymentDeclined",
				"PaymentDue",
				"PaymentPastDue",
			];
			deserializer.deserialize_enum("PaymentStatusType", VARIANTS, EnumerationVisitor)
		}
	}
}

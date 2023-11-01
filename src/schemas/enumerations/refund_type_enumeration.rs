/// <https://schema.org/RefundTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum RefundTypeEnumeration {
	/// <https://schema.org/ExchangeRefund>
	ExchangeRefund,
	/// <https://schema.org/FullRefund>
	FullRefund,
	/// <https://schema.org/StoreCreditRefund>
	StoreCreditRefund,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for RefundTypeEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				RefundTypeEnumeration::ExchangeRefund => serializer.serialize_unit_variant(
					"RefundTypeEnumeration",
					0u32,
					"ExchangeRefund",
				),
				RefundTypeEnumeration::FullRefund => {
					serializer.serialize_unit_variant("RefundTypeEnumeration", 1u32, "FullRefund")
				}
				RefundTypeEnumeration::StoreCreditRefund => serializer.serialize_unit_variant(
					"RefundTypeEnumeration",
					2u32,
					"StoreCreditRefund",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for RefundTypeEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ExchangeRefund,
				FullRefund,
				StoreCreditRefund,
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
						"ExchangeRefund" => Ok(Field::ExchangeRefund),
						"FullRefund" => Ok(Field::FullRefund),
						"StoreCreditRefund" => Ok(Field::StoreCreditRefund),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ExchangeRefund" => Ok(Field::ExchangeRefund),
						b"FullRefund" => Ok(Field::FullRefund),
						b"StoreCreditRefund" => Ok(Field::StoreCreditRefund),
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
				type Value = RefundTypeEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema RefundTypeEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ExchangeRefund, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RefundTypeEnumeration::ExchangeRefund)
						}
						(Field::FullRefund, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RefundTypeEnumeration::FullRefund)
						}
						(Field::StoreCreditRefund, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RefundTypeEnumeration::StoreCreditRefund)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["ExchangeRefund", "FullRefund", "StoreCreditRefund"];
			deserializer.deserialize_enum("RefundTypeEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}

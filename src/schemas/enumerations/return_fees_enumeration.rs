/// <https://schema.org/ReturnFeesEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ReturnFeesEnumeration {
	/// <https://schema.org/FreeReturn>
	FreeReturn,
	/// <https://schema.org/OriginalShippingFees>
	OriginalShippingFees,
	/// <https://schema.org/RestockingFees>
	RestockingFees,
	/// <https://schema.org/ReturnFeesCustomerResponsibility>
	ReturnFeesCustomerResponsibility,
	/// <https://schema.org/ReturnShippingFees>
	ReturnShippingFees,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ReturnFeesEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ReturnFeesEnumeration::FreeReturn => {
					serializer.serialize_unit_variant("ReturnFeesEnumeration", 0u32, "FreeReturn")
				}
				ReturnFeesEnumeration::OriginalShippingFees => serializer.serialize_unit_variant(
					"ReturnFeesEnumeration",
					1u32,
					"OriginalShippingFees",
				),
				ReturnFeesEnumeration::RestockingFees => serializer.serialize_unit_variant(
					"ReturnFeesEnumeration",
					2u32,
					"RestockingFees",
				),
				ReturnFeesEnumeration::ReturnFeesCustomerResponsibility => serializer
					.serialize_unit_variant(
						"ReturnFeesEnumeration",
						3u32,
						"ReturnFeesCustomerResponsibility",
					),
				ReturnFeesEnumeration::ReturnShippingFees => serializer.serialize_unit_variant(
					"ReturnFeesEnumeration",
					4u32,
					"ReturnShippingFees",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for ReturnFeesEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				FreeReturn,
				OriginalShippingFees,
				RestockingFees,
				ReturnFeesCustomerResponsibility,
				ReturnShippingFees,
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
						"FreeReturn" => Ok(Field::FreeReturn),
						"OriginalShippingFees" => Ok(Field::OriginalShippingFees),
						"RestockingFees" => Ok(Field::RestockingFees),
						"ReturnFeesCustomerResponsibility" => {
							Ok(Field::ReturnFeesCustomerResponsibility)
						}
						"ReturnShippingFees" => Ok(Field::ReturnShippingFees),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"FreeReturn" => Ok(Field::FreeReturn),
						b"OriginalShippingFees" => Ok(Field::OriginalShippingFees),
						b"RestockingFees" => Ok(Field::RestockingFees),
						b"ReturnFeesCustomerResponsibility" => {
							Ok(Field::ReturnFeesCustomerResponsibility)
						}
						b"ReturnShippingFees" => Ok(Field::ReturnShippingFees),
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
				type Value = ReturnFeesEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ReturnFeesEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::FreeReturn, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnFeesEnumeration::FreeReturn)
						}
						(Field::OriginalShippingFees, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnFeesEnumeration::OriginalShippingFees)
						}
						(Field::RestockingFees, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnFeesEnumeration::RestockingFees)
						}
						(Field::ReturnFeesCustomerResponsibility, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnFeesEnumeration::ReturnFeesCustomerResponsibility)
						}
						(Field::ReturnShippingFees, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnFeesEnumeration::ReturnShippingFees)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"FreeReturn",
				"OriginalShippingFees",
				"RestockingFees",
				"ReturnFeesCustomerResponsibility",
				"ReturnShippingFees",
			];
			deserializer.deserialize_enum("ReturnFeesEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}

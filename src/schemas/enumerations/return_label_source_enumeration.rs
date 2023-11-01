/// <https://schema.org/ReturnLabelSourceEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ReturnLabelSourceEnumeration {
	/// <https://schema.org/ReturnLabelCustomerResponsibility>
	ReturnLabelCustomerResponsibility,
	/// <https://schema.org/ReturnLabelDownloadAndPrint>
	ReturnLabelDownloadAndPrint,
	/// <https://schema.org/ReturnLabelInBox>
	ReturnLabelInBox,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ReturnLabelSourceEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ReturnLabelSourceEnumeration::ReturnLabelCustomerResponsibility => serializer
					.serialize_unit_variant(
						"ReturnLabelSourceEnumeration",
						0u32,
						"ReturnLabelCustomerResponsibility",
					),
				ReturnLabelSourceEnumeration::ReturnLabelDownloadAndPrint => serializer
					.serialize_unit_variant(
						"ReturnLabelSourceEnumeration",
						1u32,
						"ReturnLabelDownloadAndPrint",
					),
				ReturnLabelSourceEnumeration::ReturnLabelInBox => serializer
					.serialize_unit_variant(
						"ReturnLabelSourceEnumeration",
						2u32,
						"ReturnLabelInBox",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for ReturnLabelSourceEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ReturnLabelCustomerResponsibility,
				ReturnLabelDownloadAndPrint,
				ReturnLabelInBox,
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
						"ReturnLabelCustomerResponsibility" => {
							Ok(Field::ReturnLabelCustomerResponsibility)
						}
						"ReturnLabelDownloadAndPrint" => Ok(Field::ReturnLabelDownloadAndPrint),
						"ReturnLabelInBox" => Ok(Field::ReturnLabelInBox),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ReturnLabelCustomerResponsibility" => {
							Ok(Field::ReturnLabelCustomerResponsibility)
						}
						b"ReturnLabelDownloadAndPrint" => Ok(Field::ReturnLabelDownloadAndPrint),
						b"ReturnLabelInBox" => Ok(Field::ReturnLabelInBox),
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
				type Value = ReturnLabelSourceEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ReturnLabelSourceEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ReturnLabelCustomerResponsibility, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnLabelSourceEnumeration::ReturnLabelCustomerResponsibility)
						}
						(Field::ReturnLabelDownloadAndPrint, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnLabelSourceEnumeration::ReturnLabelDownloadAndPrint)
						}
						(Field::ReturnLabelInBox, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnLabelSourceEnumeration::ReturnLabelInBox)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ReturnLabelCustomerResponsibility",
				"ReturnLabelDownloadAndPrint",
				"ReturnLabelInBox",
			];
			deserializer.deserialize_enum(
				"ReturnLabelSourceEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

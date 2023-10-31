/// <https://schema.org/ReturnMethodEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ReturnMethodEnumeration {
	/// <https://schema.org/ReturnAtKiosk>
	ReturnAtKiosk,
	/// <https://schema.org/ReturnByMail>
	ReturnByMail,
	/// <https://schema.org/ReturnInStore>
	ReturnInStore,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ReturnMethodEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ReturnMethodEnumeration::ReturnAtKiosk => serializer.serialize_unit_variant(
					"ReturnMethodEnumeration",
					0u32,
					"ReturnAtKiosk",
				),
				ReturnMethodEnumeration::ReturnByMail => serializer.serialize_unit_variant(
					"ReturnMethodEnumeration",
					1u32,
					"ReturnByMail",
				),
				ReturnMethodEnumeration::ReturnInStore => serializer.serialize_unit_variant(
					"ReturnMethodEnumeration",
					2u32,
					"ReturnInStore",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for ReturnMethodEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ReturnAtKiosk,
				ReturnByMail,
				ReturnInStore,
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
						"ReturnAtKiosk" => Ok(Field::ReturnAtKiosk),
						"ReturnByMail" => Ok(Field::ReturnByMail),
						"ReturnInStore" => Ok(Field::ReturnInStore),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ReturnAtKiosk" => Ok(Field::ReturnAtKiosk),
						b"ReturnByMail" => Ok(Field::ReturnByMail),
						b"ReturnInStore" => Ok(Field::ReturnInStore),
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
				type Value = ReturnMethodEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ReturnMethodEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ReturnAtKiosk, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnMethodEnumeration::ReturnAtKiosk)
						}
						(Field::ReturnByMail, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnMethodEnumeration::ReturnByMail)
						}
						(Field::ReturnInStore, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReturnMethodEnumeration::ReturnInStore)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["ReturnAtKiosk", "ReturnByMail", "ReturnInStore"];
			deserializer.deserialize_enum("ReturnMethodEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/DigitalDocumentPermissionType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DigitalDocumentPermissionType {
	/// <https://schema.org/CommentPermission>
	CommentPermission,
	/// <https://schema.org/ReadPermission>
	ReadPermission,
	/// <https://schema.org/WritePermission>
	WritePermission,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DigitalDocumentPermissionType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DigitalDocumentPermissionType::CommentPermission => serializer
					.serialize_unit_variant(
						"DigitalDocumentPermissionType",
						0u32,
						"CommentPermission",
					),
				DigitalDocumentPermissionType::ReadPermission => serializer.serialize_unit_variant(
					"DigitalDocumentPermissionType",
					1u32,
					"ReadPermission",
				),
				DigitalDocumentPermissionType::WritePermission => serializer
					.serialize_unit_variant(
						"DigitalDocumentPermissionType",
						2u32,
						"WritePermission",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for DigitalDocumentPermissionType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				CommentPermission,
				ReadPermission,
				WritePermission,
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
						"CommentPermission" => Ok(Field::CommentPermission),
						"ReadPermission" => Ok(Field::ReadPermission),
						"WritePermission" => Ok(Field::WritePermission),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"CommentPermission" => Ok(Field::CommentPermission),
						b"ReadPermission" => Ok(Field::ReadPermission),
						b"WritePermission" => Ok(Field::WritePermission),
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
				type Value = DigitalDocumentPermissionType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DigitalDocumentPermissionType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::CommentPermission, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DigitalDocumentPermissionType::CommentPermission)
						}
						(Field::ReadPermission, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DigitalDocumentPermissionType::ReadPermission)
						}
						(Field::WritePermission, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DigitalDocumentPermissionType::WritePermission)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["CommentPermission", "ReadPermission", "WritePermission"];
			deserializer.deserialize_enum(
				"DigitalDocumentPermissionType",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

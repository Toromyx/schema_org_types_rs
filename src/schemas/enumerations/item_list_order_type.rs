/// <https://schema.org/ItemListOrderType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ItemListOrderType {
	/// <https://schema.org/ItemListOrderAscending>
	ItemListOrderAscending,
	/// <https://schema.org/ItemListOrderDescending>
	ItemListOrderDescending,
	/// <https://schema.org/ItemListUnordered>
	ItemListUnordered,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ItemListOrderType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ItemListOrderType::ItemListOrderAscending => serializer.serialize_unit_variant(
					"ItemListOrderType",
					0u32,
					"ItemListOrderAscending",
				),
				ItemListOrderType::ItemListOrderDescending => serializer.serialize_unit_variant(
					"ItemListOrderType",
					1u32,
					"ItemListOrderDescending",
				),
				ItemListOrderType::ItemListUnordered => serializer.serialize_unit_variant(
					"ItemListOrderType",
					2u32,
					"ItemListUnordered",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for ItemListOrderType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ItemListOrderAscending,
				ItemListOrderDescending,
				ItemListUnordered,
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
						"ItemListOrderAscending" => Ok(Field::ItemListOrderAscending),
						"ItemListOrderDescending" => Ok(Field::ItemListOrderDescending),
						"ItemListUnordered" => Ok(Field::ItemListUnordered),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ItemListOrderAscending" => Ok(Field::ItemListOrderAscending),
						b"ItemListOrderDescending" => Ok(Field::ItemListOrderDescending),
						b"ItemListUnordered" => Ok(Field::ItemListUnordered),
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
				type Value = ItemListOrderType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ItemListOrderType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ItemListOrderAscending, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemListOrderType::ItemListOrderAscending)
						}
						(Field::ItemListOrderDescending, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemListOrderType::ItemListOrderDescending)
						}
						(Field::ItemListUnordered, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemListOrderType::ItemListUnordered)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ItemListOrderAscending",
				"ItemListOrderDescending",
				"ItemListUnordered",
			];
			deserializer.deserialize_enum("ItemListOrderType", VARIANTS, EnumerationVisitor)
		}
	}
}

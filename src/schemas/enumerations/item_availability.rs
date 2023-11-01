/// <https://schema.org/ItemAvailability>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ItemAvailability {
	/// <https://schema.org/BackOrder>
	BackOrder,
	/// <https://schema.org/Discontinued>
	Discontinued,
	/// <https://schema.org/InStock>
	InStock,
	/// <https://schema.org/InStoreOnly>
	InStoreOnly,
	/// <https://schema.org/LimitedAvailability>
	LimitedAvailability,
	/// <https://schema.org/OnlineOnly>
	OnlineOnly,
	/// <https://schema.org/OutOfStock>
	OutOfStock,
	/// <https://schema.org/PreOrder>
	PreOrder,
	/// <https://schema.org/PreSale>
	PreSale,
	/// <https://schema.org/SoldOut>
	SoldOut,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ItemAvailability {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ItemAvailability::BackOrder => {
					serializer.serialize_unit_variant("ItemAvailability", 0u32, "BackOrder")
				}
				ItemAvailability::Discontinued => {
					serializer.serialize_unit_variant("ItemAvailability", 1u32, "Discontinued")
				}
				ItemAvailability::InStock => {
					serializer.serialize_unit_variant("ItemAvailability", 2u32, "InStock")
				}
				ItemAvailability::InStoreOnly => {
					serializer.serialize_unit_variant("ItemAvailability", 3u32, "InStoreOnly")
				}
				ItemAvailability::LimitedAvailability => serializer.serialize_unit_variant(
					"ItemAvailability",
					4u32,
					"LimitedAvailability",
				),
				ItemAvailability::OnlineOnly => {
					serializer.serialize_unit_variant("ItemAvailability", 5u32, "OnlineOnly")
				}
				ItemAvailability::OutOfStock => {
					serializer.serialize_unit_variant("ItemAvailability", 6u32, "OutOfStock")
				}
				ItemAvailability::PreOrder => {
					serializer.serialize_unit_variant("ItemAvailability", 7u32, "PreOrder")
				}
				ItemAvailability::PreSale => {
					serializer.serialize_unit_variant("ItemAvailability", 8u32, "PreSale")
				}
				ItemAvailability::SoldOut => {
					serializer.serialize_unit_variant("ItemAvailability", 9u32, "SoldOut")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for ItemAvailability {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				BackOrder,
				Discontinued,
				InStock,
				InStoreOnly,
				LimitedAvailability,
				OnlineOnly,
				OutOfStock,
				PreOrder,
				PreSale,
				SoldOut,
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
						"BackOrder" => Ok(Field::BackOrder),
						"Discontinued" => Ok(Field::Discontinued),
						"InStock" => Ok(Field::InStock),
						"InStoreOnly" => Ok(Field::InStoreOnly),
						"LimitedAvailability" => Ok(Field::LimitedAvailability),
						"OnlineOnly" => Ok(Field::OnlineOnly),
						"OutOfStock" => Ok(Field::OutOfStock),
						"PreOrder" => Ok(Field::PreOrder),
						"PreSale" => Ok(Field::PreSale),
						"SoldOut" => Ok(Field::SoldOut),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"BackOrder" => Ok(Field::BackOrder),
						b"Discontinued" => Ok(Field::Discontinued),
						b"InStock" => Ok(Field::InStock),
						b"InStoreOnly" => Ok(Field::InStoreOnly),
						b"LimitedAvailability" => Ok(Field::LimitedAvailability),
						b"OnlineOnly" => Ok(Field::OnlineOnly),
						b"OutOfStock" => Ok(Field::OutOfStock),
						b"PreOrder" => Ok(Field::PreOrder),
						b"PreSale" => Ok(Field::PreSale),
						b"SoldOut" => Ok(Field::SoldOut),
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
				type Value = ItemAvailability;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ItemAvailability")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::BackOrder, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::BackOrder)
						}
						(Field::Discontinued, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::Discontinued)
						}
						(Field::InStock, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::InStock)
						}
						(Field::InStoreOnly, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::InStoreOnly)
						}
						(Field::LimitedAvailability, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::LimitedAvailability)
						}
						(Field::OnlineOnly, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::OnlineOnly)
						}
						(Field::OutOfStock, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::OutOfStock)
						}
						(Field::PreOrder, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::PreOrder)
						}
						(Field::PreSale, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::PreSale)
						}
						(Field::SoldOut, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ItemAvailability::SoldOut)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"BackOrder",
				"Discontinued",
				"InStock",
				"InStoreOnly",
				"LimitedAvailability",
				"OnlineOnly",
				"OutOfStock",
				"PreOrder",
				"PreSale",
				"SoldOut",
			];
			deserializer.deserialize_enum("ItemAvailability", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/ProductReturnEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnEnumeration>."]
pub enum ProductReturnEnumeration {
	/// <https://schema.org/ProductReturnFiniteReturnWindow>
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnFiniteReturnWindow>."]
	ProductReturnFiniteReturnWindow,
	/// <https://schema.org/ProductReturnNotPermitted>
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnNotPermitted>."]
	ProductReturnNotPermitted,
	/// <https://schema.org/ProductReturnUnlimitedWindow>
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnlimitedWindow>."]
	ProductReturnUnlimitedWindow,
	/// <https://schema.org/ProductReturnUnspecified>
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnspecified>."]
	ProductReturnUnspecified,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ProductReturnEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ProductReturnEnumeration::ProductReturnFiniteReturnWindow => serializer
					.serialize_unit_variant(
						"ProductReturnEnumeration",
						0u32,
						"ProductReturnFiniteReturnWindow",
					),
				ProductReturnEnumeration::ProductReturnNotPermitted => serializer
					.serialize_unit_variant(
						"ProductReturnEnumeration",
						1u32,
						"ProductReturnNotPermitted",
					),
				ProductReturnEnumeration::ProductReturnUnlimitedWindow => serializer
					.serialize_unit_variant(
						"ProductReturnEnumeration",
						2u32,
						"ProductReturnUnlimitedWindow",
					),
				ProductReturnEnumeration::ProductReturnUnspecified => serializer
					.serialize_unit_variant(
						"ProductReturnEnumeration",
						3u32,
						"ProductReturnUnspecified",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for ProductReturnEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ProductReturnFiniteReturnWindow,
				ProductReturnNotPermitted,
				ProductReturnUnlimitedWindow,
				ProductReturnUnspecified,
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
						"ProductReturnFiniteReturnWindow" => {
							Ok(Field::ProductReturnFiniteReturnWindow)
						}
						"ProductReturnNotPermitted" => Ok(Field::ProductReturnNotPermitted),
						"ProductReturnUnlimitedWindow" => Ok(Field::ProductReturnUnlimitedWindow),
						"ProductReturnUnspecified" => Ok(Field::ProductReturnUnspecified),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ProductReturnFiniteReturnWindow" => {
							Ok(Field::ProductReturnFiniteReturnWindow)
						}
						b"ProductReturnNotPermitted" => Ok(Field::ProductReturnNotPermitted),
						b"ProductReturnUnlimitedWindow" => Ok(Field::ProductReturnUnlimitedWindow),
						b"ProductReturnUnspecified" => Ok(Field::ProductReturnUnspecified),
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
				type Value = ProductReturnEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ProductReturnEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ProductReturnFiniteReturnWindow, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ProductReturnEnumeration::ProductReturnFiniteReturnWindow)
						}
						(Field::ProductReturnNotPermitted, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ProductReturnEnumeration::ProductReturnNotPermitted)
						}
						(Field::ProductReturnUnlimitedWindow, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ProductReturnEnumeration::ProductReturnUnlimitedWindow)
						}
						(Field::ProductReturnUnspecified, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ProductReturnEnumeration::ProductReturnUnspecified)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ProductReturnFiniteReturnWindow",
				"ProductReturnNotPermitted",
				"ProductReturnUnlimitedWindow",
				"ProductReturnUnspecified",
			];
			deserializer.deserialize_enum("ProductReturnEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}

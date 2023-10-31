/// <https://schema.org/MerchantReturnEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MerchantReturnEnumeration {
	/// <https://schema.org/MerchantReturnFiniteReturnWindow>
	MerchantReturnFiniteReturnWindow,
	/// <https://schema.org/MerchantReturnNotPermitted>
	MerchantReturnNotPermitted,
	/// <https://schema.org/MerchantReturnUnlimitedWindow>
	MerchantReturnUnlimitedWindow,
	/// <https://schema.org/MerchantReturnUnspecified>
	MerchantReturnUnspecified,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MerchantReturnEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow => serializer
					.serialize_unit_variant(
						"MerchantReturnEnumeration",
						0u32,
						"MerchantReturnFiniteReturnWindow",
					),
				MerchantReturnEnumeration::MerchantReturnNotPermitted => serializer
					.serialize_unit_variant(
						"MerchantReturnEnumeration",
						1u32,
						"MerchantReturnNotPermitted",
					),
				MerchantReturnEnumeration::MerchantReturnUnlimitedWindow => serializer
					.serialize_unit_variant(
						"MerchantReturnEnumeration",
						2u32,
						"MerchantReturnUnlimitedWindow",
					),
				MerchantReturnEnumeration::MerchantReturnUnspecified => serializer
					.serialize_unit_variant(
						"MerchantReturnEnumeration",
						3u32,
						"MerchantReturnUnspecified",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for MerchantReturnEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				MerchantReturnFiniteReturnWindow,
				MerchantReturnNotPermitted,
				MerchantReturnUnlimitedWindow,
				MerchantReturnUnspecified,
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
						"MerchantReturnFiniteReturnWindow" => {
							Ok(Field::MerchantReturnFiniteReturnWindow)
						}
						"MerchantReturnNotPermitted" => Ok(Field::MerchantReturnNotPermitted),
						"MerchantReturnUnlimitedWindow" => Ok(Field::MerchantReturnUnlimitedWindow),
						"MerchantReturnUnspecified" => Ok(Field::MerchantReturnUnspecified),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"MerchantReturnFiniteReturnWindow" => {
							Ok(Field::MerchantReturnFiniteReturnWindow)
						}
						b"MerchantReturnNotPermitted" => Ok(Field::MerchantReturnNotPermitted),
						b"MerchantReturnUnlimitedWindow" => {
							Ok(Field::MerchantReturnUnlimitedWindow)
						}
						b"MerchantReturnUnspecified" => Ok(Field::MerchantReturnUnspecified),
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
				type Value = MerchantReturnEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MerchantReturnEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::MerchantReturnFiniteReturnWindow, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow)
						}
						(Field::MerchantReturnNotPermitted, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MerchantReturnEnumeration::MerchantReturnNotPermitted)
						}
						(Field::MerchantReturnUnlimitedWindow, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MerchantReturnEnumeration::MerchantReturnUnlimitedWindow)
						}
						(Field::MerchantReturnUnspecified, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MerchantReturnEnumeration::MerchantReturnUnspecified)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"MerchantReturnFiniteReturnWindow",
				"MerchantReturnNotPermitted",
				"MerchantReturnUnlimitedWindow",
				"MerchantReturnUnspecified",
			];
			deserializer.deserialize_enum("MerchantReturnEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}

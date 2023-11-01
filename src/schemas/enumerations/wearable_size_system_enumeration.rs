/// <https://schema.org/WearableSizeSystemEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum WearableSizeSystemEnumeration {
	/// <https://schema.org/WearableSizeSystemAU>
	WearableSizeSystemAu,
	/// <https://schema.org/WearableSizeSystemBR>
	WearableSizeSystemBr,
	/// <https://schema.org/WearableSizeSystemCN>
	WearableSizeSystemCn,
	/// <https://schema.org/WearableSizeSystemContinental>
	WearableSizeSystemContinental,
	/// <https://schema.org/WearableSizeSystemDE>
	WearableSizeSystemDe,
	/// <https://schema.org/WearableSizeSystemEN13402>
	WearableSizeSystemEn13402,
	/// <https://schema.org/WearableSizeSystemEurope>
	WearableSizeSystemEurope,
	/// <https://schema.org/WearableSizeSystemFR>
	WearableSizeSystemFr,
	/// <https://schema.org/WearableSizeSystemGS1>
	WearableSizeSystemGs1,
	/// <https://schema.org/WearableSizeSystemIT>
	WearableSizeSystemIt,
	/// <https://schema.org/WearableSizeSystemJP>
	WearableSizeSystemJp,
	/// <https://schema.org/WearableSizeSystemMX>
	WearableSizeSystemMx,
	/// <https://schema.org/WearableSizeSystemUK>
	WearableSizeSystemUk,
	/// <https://schema.org/WearableSizeSystemUS>
	WearableSizeSystemUs,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for WearableSizeSystemEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				WearableSizeSystemEnumeration::WearableSizeSystemAu => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						0u32,
						"WearableSizeSystemAu",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemBr => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						1u32,
						"WearableSizeSystemBr",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemCn => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						2u32,
						"WearableSizeSystemCn",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemContinental => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						3u32,
						"WearableSizeSystemContinental",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemDe => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						4u32,
						"WearableSizeSystemDe",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemEn13402 => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						5u32,
						"WearableSizeSystemEn13402",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemEurope => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						6u32,
						"WearableSizeSystemEurope",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemFr => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						7u32,
						"WearableSizeSystemFr",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemGs1 => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						8u32,
						"WearableSizeSystemGs1",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemIt => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						9u32,
						"WearableSizeSystemIt",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemJp => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						10u32,
						"WearableSizeSystemJp",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemMx => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						11u32,
						"WearableSizeSystemMx",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemUk => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						12u32,
						"WearableSizeSystemUk",
					),
				WearableSizeSystemEnumeration::WearableSizeSystemUs => serializer
					.serialize_unit_variant(
						"WearableSizeSystemEnumeration",
						13u32,
						"WearableSizeSystemUs",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for WearableSizeSystemEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				WearableSizeSystemAu,
				WearableSizeSystemBr,
				WearableSizeSystemCn,
				WearableSizeSystemContinental,
				WearableSizeSystemDe,
				WearableSizeSystemEn13402,
				WearableSizeSystemEurope,
				WearableSizeSystemFr,
				WearableSizeSystemGs1,
				WearableSizeSystemIt,
				WearableSizeSystemJp,
				WearableSizeSystemMx,
				WearableSizeSystemUk,
				WearableSizeSystemUs,
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
						"WearableSizeSystemAu" => Ok(Field::WearableSizeSystemAu),
						"WearableSizeSystemBr" => Ok(Field::WearableSizeSystemBr),
						"WearableSizeSystemCn" => Ok(Field::WearableSizeSystemCn),
						"WearableSizeSystemContinental" => Ok(Field::WearableSizeSystemContinental),
						"WearableSizeSystemDe" => Ok(Field::WearableSizeSystemDe),
						"WearableSizeSystemEn13402" => Ok(Field::WearableSizeSystemEn13402),
						"WearableSizeSystemEurope" => Ok(Field::WearableSizeSystemEurope),
						"WearableSizeSystemFr" => Ok(Field::WearableSizeSystemFr),
						"WearableSizeSystemGs1" => Ok(Field::WearableSizeSystemGs1),
						"WearableSizeSystemIt" => Ok(Field::WearableSizeSystemIt),
						"WearableSizeSystemJp" => Ok(Field::WearableSizeSystemJp),
						"WearableSizeSystemMx" => Ok(Field::WearableSizeSystemMx),
						"WearableSizeSystemUk" => Ok(Field::WearableSizeSystemUk),
						"WearableSizeSystemUs" => Ok(Field::WearableSizeSystemUs),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"WearableSizeSystemAu" => Ok(Field::WearableSizeSystemAu),
						b"WearableSizeSystemBr" => Ok(Field::WearableSizeSystemBr),
						b"WearableSizeSystemCn" => Ok(Field::WearableSizeSystemCn),
						b"WearableSizeSystemContinental" => {
							Ok(Field::WearableSizeSystemContinental)
						}
						b"WearableSizeSystemDe" => Ok(Field::WearableSizeSystemDe),
						b"WearableSizeSystemEn13402" => Ok(Field::WearableSizeSystemEn13402),
						b"WearableSizeSystemEurope" => Ok(Field::WearableSizeSystemEurope),
						b"WearableSizeSystemFr" => Ok(Field::WearableSizeSystemFr),
						b"WearableSizeSystemGs1" => Ok(Field::WearableSizeSystemGs1),
						b"WearableSizeSystemIt" => Ok(Field::WearableSizeSystemIt),
						b"WearableSizeSystemJp" => Ok(Field::WearableSizeSystemJp),
						b"WearableSizeSystemMx" => Ok(Field::WearableSizeSystemMx),
						b"WearableSizeSystemUk" => Ok(Field::WearableSizeSystemUk),
						b"WearableSizeSystemUs" => Ok(Field::WearableSizeSystemUs),
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
				type Value = WearableSizeSystemEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema WearableSizeSystemEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::WearableSizeSystemAu, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemAu)
						}
						(Field::WearableSizeSystemBr, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemBr)
						}
						(Field::WearableSizeSystemCn, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemCn)
						}
						(Field::WearableSizeSystemContinental, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemContinental)
						}
						(Field::WearableSizeSystemDe, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemDe)
						}
						(Field::WearableSizeSystemEn13402, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemEn13402)
						}
						(Field::WearableSizeSystemEurope, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemEurope)
						}
						(Field::WearableSizeSystemFr, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemFr)
						}
						(Field::WearableSizeSystemGs1, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemGs1)
						}
						(Field::WearableSizeSystemIt, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemIt)
						}
						(Field::WearableSizeSystemJp, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemJp)
						}
						(Field::WearableSizeSystemMx, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemMx)
						}
						(Field::WearableSizeSystemUk, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemUk)
						}
						(Field::WearableSizeSystemUs, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeSystemEnumeration::WearableSizeSystemUs)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"WearableSizeSystemAu",
				"WearableSizeSystemBr",
				"WearableSizeSystemCn",
				"WearableSizeSystemContinental",
				"WearableSizeSystemDe",
				"WearableSizeSystemEn13402",
				"WearableSizeSystemEurope",
				"WearableSizeSystemFr",
				"WearableSizeSystemGs1",
				"WearableSizeSystemIt",
				"WearableSizeSystemJp",
				"WearableSizeSystemMx",
				"WearableSizeSystemUk",
				"WearableSizeSystemUs",
			];
			deserializer.deserialize_enum(
				"WearableSizeSystemEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

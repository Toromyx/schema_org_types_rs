/// <https://schema.org/EUEnergyEfficiencyEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum EuEnergyEfficiencyEnumeration {
	/// <https://schema.org/EUEnergyEfficiencyCategoryA>
	EuEnergyEfficiencyCategoryA,
	/// <https://schema.org/EUEnergyEfficiencyCategoryA1Plus>
	EuEnergyEfficiencyCategoryA1Plus,
	/// <https://schema.org/EUEnergyEfficiencyCategoryA2Plus>
	EuEnergyEfficiencyCategoryA2Plus,
	/// <https://schema.org/EUEnergyEfficiencyCategoryA3Plus>
	EuEnergyEfficiencyCategoryA3Plus,
	/// <https://schema.org/EUEnergyEfficiencyCategoryB>
	EuEnergyEfficiencyCategoryB,
	/// <https://schema.org/EUEnergyEfficiencyCategoryC>
	EuEnergyEfficiencyCategoryC,
	/// <https://schema.org/EUEnergyEfficiencyCategoryD>
	EuEnergyEfficiencyCategoryD,
	/// <https://schema.org/EUEnergyEfficiencyCategoryE>
	EuEnergyEfficiencyCategoryE,
	/// <https://schema.org/EUEnergyEfficiencyCategoryF>
	EuEnergyEfficiencyCategoryF,
	/// <https://schema.org/EUEnergyEfficiencyCategoryG>
	EuEnergyEfficiencyCategoryG,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for EuEnergyEfficiencyEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryA => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						0u32,
						"EuEnergyEfficiencyCategoryA",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryA1Plus => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						1u32,
						"EuEnergyEfficiencyCategoryA1Plus",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryA2Plus => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						2u32,
						"EuEnergyEfficiencyCategoryA2Plus",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryA3Plus => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						3u32,
						"EuEnergyEfficiencyCategoryA3Plus",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryB => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						4u32,
						"EuEnergyEfficiencyCategoryB",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryC => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						5u32,
						"EuEnergyEfficiencyCategoryC",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryD => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						6u32,
						"EuEnergyEfficiencyCategoryD",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryE => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						7u32,
						"EuEnergyEfficiencyCategoryE",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryF => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						8u32,
						"EuEnergyEfficiencyCategoryF",
					),
				EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryG => serializer
					.serialize_unit_variant(
						"EuEnergyEfficiencyEnumeration",
						9u32,
						"EuEnergyEfficiencyCategoryG",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for EuEnergyEfficiencyEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				EuEnergyEfficiencyCategoryA,
				EuEnergyEfficiencyCategoryA1Plus,
				EuEnergyEfficiencyCategoryA2Plus,
				EuEnergyEfficiencyCategoryA3Plus,
				EuEnergyEfficiencyCategoryB,
				EuEnergyEfficiencyCategoryC,
				EuEnergyEfficiencyCategoryD,
				EuEnergyEfficiencyCategoryE,
				EuEnergyEfficiencyCategoryF,
				EuEnergyEfficiencyCategoryG,
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
						"EuEnergyEfficiencyCategoryA" => Ok(Field::EuEnergyEfficiencyCategoryA),
						"EuEnergyEfficiencyCategoryA1Plus" => {
							Ok(Field::EuEnergyEfficiencyCategoryA1Plus)
						}
						"EuEnergyEfficiencyCategoryA2Plus" => {
							Ok(Field::EuEnergyEfficiencyCategoryA2Plus)
						}
						"EuEnergyEfficiencyCategoryA3Plus" => {
							Ok(Field::EuEnergyEfficiencyCategoryA3Plus)
						}
						"EuEnergyEfficiencyCategoryB" => Ok(Field::EuEnergyEfficiencyCategoryB),
						"EuEnergyEfficiencyCategoryC" => Ok(Field::EuEnergyEfficiencyCategoryC),
						"EuEnergyEfficiencyCategoryD" => Ok(Field::EuEnergyEfficiencyCategoryD),
						"EuEnergyEfficiencyCategoryE" => Ok(Field::EuEnergyEfficiencyCategoryE),
						"EuEnergyEfficiencyCategoryF" => Ok(Field::EuEnergyEfficiencyCategoryF),
						"EuEnergyEfficiencyCategoryG" => Ok(Field::EuEnergyEfficiencyCategoryG),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"EuEnergyEfficiencyCategoryA" => Ok(Field::EuEnergyEfficiencyCategoryA),
						b"EuEnergyEfficiencyCategoryA1Plus" => {
							Ok(Field::EuEnergyEfficiencyCategoryA1Plus)
						}
						b"EuEnergyEfficiencyCategoryA2Plus" => {
							Ok(Field::EuEnergyEfficiencyCategoryA2Plus)
						}
						b"EuEnergyEfficiencyCategoryA3Plus" => {
							Ok(Field::EuEnergyEfficiencyCategoryA3Plus)
						}
						b"EuEnergyEfficiencyCategoryB" => Ok(Field::EuEnergyEfficiencyCategoryB),
						b"EuEnergyEfficiencyCategoryC" => Ok(Field::EuEnergyEfficiencyCategoryC),
						b"EuEnergyEfficiencyCategoryD" => Ok(Field::EuEnergyEfficiencyCategoryD),
						b"EuEnergyEfficiencyCategoryE" => Ok(Field::EuEnergyEfficiencyCategoryE),
						b"EuEnergyEfficiencyCategoryF" => Ok(Field::EuEnergyEfficiencyCategoryF),
						b"EuEnergyEfficiencyCategoryG" => Ok(Field::EuEnergyEfficiencyCategoryG),
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
				type Value = EuEnergyEfficiencyEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema EUEnergyEfficiencyEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::EuEnergyEfficiencyCategoryA, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryA)
						}
						(Field::EuEnergyEfficiencyCategoryA1Plus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryA1Plus)
						}
						(Field::EuEnergyEfficiencyCategoryA2Plus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryA2Plus)
						}
						(Field::EuEnergyEfficiencyCategoryA3Plus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryA3Plus)
						}
						(Field::EuEnergyEfficiencyCategoryB, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryB)
						}
						(Field::EuEnergyEfficiencyCategoryC, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryC)
						}
						(Field::EuEnergyEfficiencyCategoryD, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryD)
						}
						(Field::EuEnergyEfficiencyCategoryE, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryE)
						}
						(Field::EuEnergyEfficiencyCategoryF, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryF)
						}
						(Field::EuEnergyEfficiencyCategoryG, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EuEnergyEfficiencyEnumeration::EuEnergyEfficiencyCategoryG)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"EuEnergyEfficiencyCategoryA",
				"EuEnergyEfficiencyCategoryA1Plus",
				"EuEnergyEfficiencyCategoryA2Plus",
				"EuEnergyEfficiencyCategoryA3Plus",
				"EuEnergyEfficiencyCategoryB",
				"EuEnergyEfficiencyCategoryC",
				"EuEnergyEfficiencyCategoryD",
				"EuEnergyEfficiencyCategoryE",
				"EuEnergyEfficiencyCategoryF",
				"EuEnergyEfficiencyCategoryG",
			];
			deserializer.deserialize_enum(
				"EuEnergyEfficiencyEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

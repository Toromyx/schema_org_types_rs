/// <https://schema.org/WearableMeasurementTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum WearableMeasurementTypeEnumeration {
	/// <https://schema.org/WearableMeasurementBack>
	WearableMeasurementBack,
	/// <https://schema.org/WearableMeasurementChestOrBust>
	WearableMeasurementChestOrBust,
	/// <https://schema.org/WearableMeasurementCollar>
	WearableMeasurementCollar,
	/// <https://schema.org/WearableMeasurementCup>
	WearableMeasurementCup,
	/// <https://schema.org/WearableMeasurementHeight>
	WearableMeasurementHeight,
	/// <https://schema.org/WearableMeasurementHips>
	WearableMeasurementHips,
	/// <https://schema.org/WearableMeasurementInseam>
	WearableMeasurementInseam,
	/// <https://schema.org/WearableMeasurementLength>
	WearableMeasurementLength,
	/// <https://schema.org/WearableMeasurementOutsideLeg>
	WearableMeasurementOutsideLeg,
	/// <https://schema.org/WearableMeasurementSleeve>
	WearableMeasurementSleeve,
	/// <https://schema.org/WearableMeasurementWaist>
	WearableMeasurementWaist,
	/// <https://schema.org/WearableMeasurementWidth>
	WearableMeasurementWidth,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for WearableMeasurementTypeEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				WearableMeasurementTypeEnumeration::WearableMeasurementBack => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						0u32,
						"WearableMeasurementBack",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementChestOrBust => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						1u32,
						"WearableMeasurementChestOrBust",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementCollar => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						2u32,
						"WearableMeasurementCollar",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementCup => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						3u32,
						"WearableMeasurementCup",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementHeight => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						4u32,
						"WearableMeasurementHeight",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementHips => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						5u32,
						"WearableMeasurementHips",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementInseam => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						6u32,
						"WearableMeasurementInseam",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementLength => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						7u32,
						"WearableMeasurementLength",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementOutsideLeg => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						8u32,
						"WearableMeasurementOutsideLeg",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementSleeve => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						9u32,
						"WearableMeasurementSleeve",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementWaist => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						10u32,
						"WearableMeasurementWaist",
					),
				WearableMeasurementTypeEnumeration::WearableMeasurementWidth => serializer
					.serialize_unit_variant(
						"WearableMeasurementTypeEnumeration",
						11u32,
						"WearableMeasurementWidth",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for WearableMeasurementTypeEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				WearableMeasurementBack,
				WearableMeasurementChestOrBust,
				WearableMeasurementCollar,
				WearableMeasurementCup,
				WearableMeasurementHeight,
				WearableMeasurementHips,
				WearableMeasurementInseam,
				WearableMeasurementLength,
				WearableMeasurementOutsideLeg,
				WearableMeasurementSleeve,
				WearableMeasurementWaist,
				WearableMeasurementWidth,
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
						"WearableMeasurementBack" => Ok(Field::WearableMeasurementBack),
						"WearableMeasurementChestOrBust" => {
							Ok(Field::WearableMeasurementChestOrBust)
						}
						"WearableMeasurementCollar" => Ok(Field::WearableMeasurementCollar),
						"WearableMeasurementCup" => Ok(Field::WearableMeasurementCup),
						"WearableMeasurementHeight" => Ok(Field::WearableMeasurementHeight),
						"WearableMeasurementHips" => Ok(Field::WearableMeasurementHips),
						"WearableMeasurementInseam" => Ok(Field::WearableMeasurementInseam),
						"WearableMeasurementLength" => Ok(Field::WearableMeasurementLength),
						"WearableMeasurementOutsideLeg" => Ok(Field::WearableMeasurementOutsideLeg),
						"WearableMeasurementSleeve" => Ok(Field::WearableMeasurementSleeve),
						"WearableMeasurementWaist" => Ok(Field::WearableMeasurementWaist),
						"WearableMeasurementWidth" => Ok(Field::WearableMeasurementWidth),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"WearableMeasurementBack" => Ok(Field::WearableMeasurementBack),
						b"WearableMeasurementChestOrBust" => {
							Ok(Field::WearableMeasurementChestOrBust)
						}
						b"WearableMeasurementCollar" => Ok(Field::WearableMeasurementCollar),
						b"WearableMeasurementCup" => Ok(Field::WearableMeasurementCup),
						b"WearableMeasurementHeight" => Ok(Field::WearableMeasurementHeight),
						b"WearableMeasurementHips" => Ok(Field::WearableMeasurementHips),
						b"WearableMeasurementInseam" => Ok(Field::WearableMeasurementInseam),
						b"WearableMeasurementLength" => Ok(Field::WearableMeasurementLength),
						b"WearableMeasurementOutsideLeg" => {
							Ok(Field::WearableMeasurementOutsideLeg)
						}
						b"WearableMeasurementSleeve" => Ok(Field::WearableMeasurementSleeve),
						b"WearableMeasurementWaist" => Ok(Field::WearableMeasurementWaist),
						b"WearableMeasurementWidth" => Ok(Field::WearableMeasurementWidth),
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
				type Value = WearableMeasurementTypeEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema WearableMeasurementTypeEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::WearableMeasurementBack, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementBack)
						}
						(Field::WearableMeasurementChestOrBust, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementChestOrBust)
						}
						(Field::WearableMeasurementCollar, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementCollar)
						}
						(Field::WearableMeasurementCup, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementCup)
						}
						(Field::WearableMeasurementHeight, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementHeight)
						}
						(Field::WearableMeasurementHips, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementHips)
						}
						(Field::WearableMeasurementInseam, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementInseam)
						}
						(Field::WearableMeasurementLength, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementLength)
						}
						(Field::WearableMeasurementOutsideLeg, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementOutsideLeg)
						}
						(Field::WearableMeasurementSleeve, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementSleeve)
						}
						(Field::WearableMeasurementWaist, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementWaist)
						}
						(Field::WearableMeasurementWidth, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableMeasurementTypeEnumeration::WearableMeasurementWidth)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"WearableMeasurementBack",
				"WearableMeasurementChestOrBust",
				"WearableMeasurementCollar",
				"WearableMeasurementCup",
				"WearableMeasurementHeight",
				"WearableMeasurementHips",
				"WearableMeasurementInseam",
				"WearableMeasurementLength",
				"WearableMeasurementOutsideLeg",
				"WearableMeasurementSleeve",
				"WearableMeasurementWaist",
				"WearableMeasurementWidth",
			];
			deserializer.deserialize_enum(
				"WearableMeasurementTypeEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

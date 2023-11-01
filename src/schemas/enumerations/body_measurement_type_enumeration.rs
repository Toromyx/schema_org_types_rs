/// <https://schema.org/BodyMeasurementTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum BodyMeasurementTypeEnumeration {
	/// <https://schema.org/BodyMeasurementArm>
	BodyMeasurementArm,
	/// <https://schema.org/BodyMeasurementBust>
	BodyMeasurementBust,
	/// <https://schema.org/BodyMeasurementChest>
	BodyMeasurementChest,
	/// <https://schema.org/BodyMeasurementFoot>
	BodyMeasurementFoot,
	/// <https://schema.org/BodyMeasurementHand>
	BodyMeasurementHand,
	/// <https://schema.org/BodyMeasurementHead>
	BodyMeasurementHead,
	/// <https://schema.org/BodyMeasurementHeight>
	BodyMeasurementHeight,
	/// <https://schema.org/BodyMeasurementHips>
	BodyMeasurementHips,
	/// <https://schema.org/BodyMeasurementInsideLeg>
	BodyMeasurementInsideLeg,
	/// <https://schema.org/BodyMeasurementNeck>
	BodyMeasurementNeck,
	/// <https://schema.org/BodyMeasurementUnderbust>
	BodyMeasurementUnderbust,
	/// <https://schema.org/BodyMeasurementWaist>
	BodyMeasurementWaist,
	/// <https://schema.org/BodyMeasurementWeight>
	BodyMeasurementWeight,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for BodyMeasurementTypeEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				BodyMeasurementTypeEnumeration::BodyMeasurementArm => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						0u32,
						"BodyMeasurementArm",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementBust => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						1u32,
						"BodyMeasurementBust",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementChest => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						2u32,
						"BodyMeasurementChest",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementFoot => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						3u32,
						"BodyMeasurementFoot",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementHand => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						4u32,
						"BodyMeasurementHand",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementHead => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						5u32,
						"BodyMeasurementHead",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementHeight => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						6u32,
						"BodyMeasurementHeight",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementHips => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						7u32,
						"BodyMeasurementHips",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementInsideLeg => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						8u32,
						"BodyMeasurementInsideLeg",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementNeck => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						9u32,
						"BodyMeasurementNeck",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementUnderbust => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						10u32,
						"BodyMeasurementUnderbust",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementWaist => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						11u32,
						"BodyMeasurementWaist",
					),
				BodyMeasurementTypeEnumeration::BodyMeasurementWeight => serializer
					.serialize_unit_variant(
						"BodyMeasurementTypeEnumeration",
						12u32,
						"BodyMeasurementWeight",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for BodyMeasurementTypeEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				BodyMeasurementArm,
				BodyMeasurementBust,
				BodyMeasurementChest,
				BodyMeasurementFoot,
				BodyMeasurementHand,
				BodyMeasurementHead,
				BodyMeasurementHeight,
				BodyMeasurementHips,
				BodyMeasurementInsideLeg,
				BodyMeasurementNeck,
				BodyMeasurementUnderbust,
				BodyMeasurementWaist,
				BodyMeasurementWeight,
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
						"BodyMeasurementArm" => Ok(Field::BodyMeasurementArm),
						"BodyMeasurementBust" => Ok(Field::BodyMeasurementBust),
						"BodyMeasurementChest" => Ok(Field::BodyMeasurementChest),
						"BodyMeasurementFoot" => Ok(Field::BodyMeasurementFoot),
						"BodyMeasurementHand" => Ok(Field::BodyMeasurementHand),
						"BodyMeasurementHead" => Ok(Field::BodyMeasurementHead),
						"BodyMeasurementHeight" => Ok(Field::BodyMeasurementHeight),
						"BodyMeasurementHips" => Ok(Field::BodyMeasurementHips),
						"BodyMeasurementInsideLeg" => Ok(Field::BodyMeasurementInsideLeg),
						"BodyMeasurementNeck" => Ok(Field::BodyMeasurementNeck),
						"BodyMeasurementUnderbust" => Ok(Field::BodyMeasurementUnderbust),
						"BodyMeasurementWaist" => Ok(Field::BodyMeasurementWaist),
						"BodyMeasurementWeight" => Ok(Field::BodyMeasurementWeight),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"BodyMeasurementArm" => Ok(Field::BodyMeasurementArm),
						b"BodyMeasurementBust" => Ok(Field::BodyMeasurementBust),
						b"BodyMeasurementChest" => Ok(Field::BodyMeasurementChest),
						b"BodyMeasurementFoot" => Ok(Field::BodyMeasurementFoot),
						b"BodyMeasurementHand" => Ok(Field::BodyMeasurementHand),
						b"BodyMeasurementHead" => Ok(Field::BodyMeasurementHead),
						b"BodyMeasurementHeight" => Ok(Field::BodyMeasurementHeight),
						b"BodyMeasurementHips" => Ok(Field::BodyMeasurementHips),
						b"BodyMeasurementInsideLeg" => Ok(Field::BodyMeasurementInsideLeg),
						b"BodyMeasurementNeck" => Ok(Field::BodyMeasurementNeck),
						b"BodyMeasurementUnderbust" => Ok(Field::BodyMeasurementUnderbust),
						b"BodyMeasurementWaist" => Ok(Field::BodyMeasurementWaist),
						b"BodyMeasurementWeight" => Ok(Field::BodyMeasurementWeight),
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
				type Value = BodyMeasurementTypeEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema BodyMeasurementTypeEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::BodyMeasurementArm, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementArm)
						}
						(Field::BodyMeasurementBust, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementBust)
						}
						(Field::BodyMeasurementChest, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementChest)
						}
						(Field::BodyMeasurementFoot, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementFoot)
						}
						(Field::BodyMeasurementHand, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementHand)
						}
						(Field::BodyMeasurementHead, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementHead)
						}
						(Field::BodyMeasurementHeight, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementHeight)
						}
						(Field::BodyMeasurementHips, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementHips)
						}
						(Field::BodyMeasurementInsideLeg, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementInsideLeg)
						}
						(Field::BodyMeasurementNeck, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementNeck)
						}
						(Field::BodyMeasurementUnderbust, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementUnderbust)
						}
						(Field::BodyMeasurementWaist, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementWaist)
						}
						(Field::BodyMeasurementWeight, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BodyMeasurementTypeEnumeration::BodyMeasurementWeight)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"BodyMeasurementArm",
				"BodyMeasurementBust",
				"BodyMeasurementChest",
				"BodyMeasurementFoot",
				"BodyMeasurementHand",
				"BodyMeasurementHead",
				"BodyMeasurementHeight",
				"BodyMeasurementHips",
				"BodyMeasurementInsideLeg",
				"BodyMeasurementNeck",
				"BodyMeasurementUnderbust",
				"BodyMeasurementWaist",
				"BodyMeasurementWeight",
			];
			deserializer.deserialize_enum(
				"BodyMeasurementTypeEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

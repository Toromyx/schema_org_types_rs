/// <https://schema.org/WearableSizeGroupEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum WearableSizeGroupEnumeration {
	/// <https://schema.org/WearableSizeGroupBig>
	WearableSizeGroupBig,
	/// <https://schema.org/WearableSizeGroupBoys>
	WearableSizeGroupBoys,
	/// <https://schema.org/WearableSizeGroupExtraShort>
	WearableSizeGroupExtraShort,
	/// <https://schema.org/WearableSizeGroupExtraTall>
	WearableSizeGroupExtraTall,
	/// <https://schema.org/WearableSizeGroupGirls>
	WearableSizeGroupGirls,
	/// <https://schema.org/WearableSizeGroupHusky>
	WearableSizeGroupHusky,
	/// <https://schema.org/WearableSizeGroupInfants>
	WearableSizeGroupInfants,
	/// <https://schema.org/WearableSizeGroupJuniors>
	WearableSizeGroupJuniors,
	/// <https://schema.org/WearableSizeGroupMaternity>
	WearableSizeGroupMaternity,
	/// <https://schema.org/WearableSizeGroupMens>
	WearableSizeGroupMens,
	/// <https://schema.org/WearableSizeGroupMisses>
	WearableSizeGroupMisses,
	/// <https://schema.org/WearableSizeGroupPetite>
	WearableSizeGroupPetite,
	/// <https://schema.org/WearableSizeGroupPlus>
	WearableSizeGroupPlus,
	/// <https://schema.org/WearableSizeGroupRegular>
	WearableSizeGroupRegular,
	/// <https://schema.org/WearableSizeGroupShort>
	WearableSizeGroupShort,
	/// <https://schema.org/WearableSizeGroupTall>
	WearableSizeGroupTall,
	/// <https://schema.org/WearableSizeGroupWomens>
	WearableSizeGroupWomens,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for WearableSizeGroupEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				WearableSizeGroupEnumeration::WearableSizeGroupBig => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						0u32,
						"WearableSizeGroupBig",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupBoys => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						1u32,
						"WearableSizeGroupBoys",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupExtraShort => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						2u32,
						"WearableSizeGroupExtraShort",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupExtraTall => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						3u32,
						"WearableSizeGroupExtraTall",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupGirls => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						4u32,
						"WearableSizeGroupGirls",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupHusky => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						5u32,
						"WearableSizeGroupHusky",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupInfants => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						6u32,
						"WearableSizeGroupInfants",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupJuniors => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						7u32,
						"WearableSizeGroupJuniors",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupMaternity => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						8u32,
						"WearableSizeGroupMaternity",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupMens => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						9u32,
						"WearableSizeGroupMens",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupMisses => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						10u32,
						"WearableSizeGroupMisses",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupPetite => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						11u32,
						"WearableSizeGroupPetite",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupPlus => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						12u32,
						"WearableSizeGroupPlus",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupRegular => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						13u32,
						"WearableSizeGroupRegular",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupShort => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						14u32,
						"WearableSizeGroupShort",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupTall => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						15u32,
						"WearableSizeGroupTall",
					),
				WearableSizeGroupEnumeration::WearableSizeGroupWomens => serializer
					.serialize_unit_variant(
						"WearableSizeGroupEnumeration",
						16u32,
						"WearableSizeGroupWomens",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for WearableSizeGroupEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				WearableSizeGroupBig,
				WearableSizeGroupBoys,
				WearableSizeGroupExtraShort,
				WearableSizeGroupExtraTall,
				WearableSizeGroupGirls,
				WearableSizeGroupHusky,
				WearableSizeGroupInfants,
				WearableSizeGroupJuniors,
				WearableSizeGroupMaternity,
				WearableSizeGroupMens,
				WearableSizeGroupMisses,
				WearableSizeGroupPetite,
				WearableSizeGroupPlus,
				WearableSizeGroupRegular,
				WearableSizeGroupShort,
				WearableSizeGroupTall,
				WearableSizeGroupWomens,
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
						"WearableSizeGroupBig" => Ok(Field::WearableSizeGroupBig),
						"WearableSizeGroupBoys" => Ok(Field::WearableSizeGroupBoys),
						"WearableSizeGroupExtraShort" => Ok(Field::WearableSizeGroupExtraShort),
						"WearableSizeGroupExtraTall" => Ok(Field::WearableSizeGroupExtraTall),
						"WearableSizeGroupGirls" => Ok(Field::WearableSizeGroupGirls),
						"WearableSizeGroupHusky" => Ok(Field::WearableSizeGroupHusky),
						"WearableSizeGroupInfants" => Ok(Field::WearableSizeGroupInfants),
						"WearableSizeGroupJuniors" => Ok(Field::WearableSizeGroupJuniors),
						"WearableSizeGroupMaternity" => Ok(Field::WearableSizeGroupMaternity),
						"WearableSizeGroupMens" => Ok(Field::WearableSizeGroupMens),
						"WearableSizeGroupMisses" => Ok(Field::WearableSizeGroupMisses),
						"WearableSizeGroupPetite" => Ok(Field::WearableSizeGroupPetite),
						"WearableSizeGroupPlus" => Ok(Field::WearableSizeGroupPlus),
						"WearableSizeGroupRegular" => Ok(Field::WearableSizeGroupRegular),
						"WearableSizeGroupShort" => Ok(Field::WearableSizeGroupShort),
						"WearableSizeGroupTall" => Ok(Field::WearableSizeGroupTall),
						"WearableSizeGroupWomens" => Ok(Field::WearableSizeGroupWomens),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"WearableSizeGroupBig" => Ok(Field::WearableSizeGroupBig),
						b"WearableSizeGroupBoys" => Ok(Field::WearableSizeGroupBoys),
						b"WearableSizeGroupExtraShort" => Ok(Field::WearableSizeGroupExtraShort),
						b"WearableSizeGroupExtraTall" => Ok(Field::WearableSizeGroupExtraTall),
						b"WearableSizeGroupGirls" => Ok(Field::WearableSizeGroupGirls),
						b"WearableSizeGroupHusky" => Ok(Field::WearableSizeGroupHusky),
						b"WearableSizeGroupInfants" => Ok(Field::WearableSizeGroupInfants),
						b"WearableSizeGroupJuniors" => Ok(Field::WearableSizeGroupJuniors),
						b"WearableSizeGroupMaternity" => Ok(Field::WearableSizeGroupMaternity),
						b"WearableSizeGroupMens" => Ok(Field::WearableSizeGroupMens),
						b"WearableSizeGroupMisses" => Ok(Field::WearableSizeGroupMisses),
						b"WearableSizeGroupPetite" => Ok(Field::WearableSizeGroupPetite),
						b"WearableSizeGroupPlus" => Ok(Field::WearableSizeGroupPlus),
						b"WearableSizeGroupRegular" => Ok(Field::WearableSizeGroupRegular),
						b"WearableSizeGroupShort" => Ok(Field::WearableSizeGroupShort),
						b"WearableSizeGroupTall" => Ok(Field::WearableSizeGroupTall),
						b"WearableSizeGroupWomens" => Ok(Field::WearableSizeGroupWomens),
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
				type Value = WearableSizeGroupEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema WearableSizeGroupEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::WearableSizeGroupBig, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupBig)
						}
						(Field::WearableSizeGroupBoys, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupBoys)
						}
						(Field::WearableSizeGroupExtraShort, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupExtraShort)
						}
						(Field::WearableSizeGroupExtraTall, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupExtraTall)
						}
						(Field::WearableSizeGroupGirls, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupGirls)
						}
						(Field::WearableSizeGroupHusky, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupHusky)
						}
						(Field::WearableSizeGroupInfants, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupInfants)
						}
						(Field::WearableSizeGroupJuniors, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupJuniors)
						}
						(Field::WearableSizeGroupMaternity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupMaternity)
						}
						(Field::WearableSizeGroupMens, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupMens)
						}
						(Field::WearableSizeGroupMisses, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupMisses)
						}
						(Field::WearableSizeGroupPetite, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupPetite)
						}
						(Field::WearableSizeGroupPlus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupPlus)
						}
						(Field::WearableSizeGroupRegular, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupRegular)
						}
						(Field::WearableSizeGroupShort, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupShort)
						}
						(Field::WearableSizeGroupTall, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupTall)
						}
						(Field::WearableSizeGroupWomens, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(WearableSizeGroupEnumeration::WearableSizeGroupWomens)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"WearableSizeGroupBig",
				"WearableSizeGroupBoys",
				"WearableSizeGroupExtraShort",
				"WearableSizeGroupExtraTall",
				"WearableSizeGroupGirls",
				"WearableSizeGroupHusky",
				"WearableSizeGroupInfants",
				"WearableSizeGroupJuniors",
				"WearableSizeGroupMaternity",
				"WearableSizeGroupMens",
				"WearableSizeGroupMisses",
				"WearableSizeGroupPetite",
				"WearableSizeGroupPlus",
				"WearableSizeGroupRegular",
				"WearableSizeGroupShort",
				"WearableSizeGroupTall",
				"WearableSizeGroupWomens",
			];
			deserializer.deserialize_enum(
				"WearableSizeGroupEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

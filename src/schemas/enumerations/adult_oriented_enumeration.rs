/// <https://schema.org/AdultOrientedEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AdultOrientedEnumeration {
	/// <https://schema.org/AlcoholConsideration>
	AlcoholConsideration,
	/// <https://schema.org/DangerousGoodConsideration>
	DangerousGoodConsideration,
	/// <https://schema.org/HealthcareConsideration>
	HealthcareConsideration,
	/// <https://schema.org/NarcoticConsideration>
	NarcoticConsideration,
	/// <https://schema.org/ReducedRelevanceForChildrenConsideration>
	ReducedRelevanceForChildrenConsideration,
	/// <https://schema.org/SexualContentConsideration>
	SexualContentConsideration,
	/// <https://schema.org/TobaccoNicotineConsideration>
	TobaccoNicotineConsideration,
	/// <https://schema.org/UnclassifiedAdultConsideration>
	UnclassifiedAdultConsideration,
	/// <https://schema.org/ViolenceConsideration>
	ViolenceConsideration,
	/// <https://schema.org/WeaponConsideration>
	WeaponConsideration,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AdultOrientedEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				AdultOrientedEnumeration::AlcoholConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						0u32,
						"AlcoholConsideration",
					),
				AdultOrientedEnumeration::DangerousGoodConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						1u32,
						"DangerousGoodConsideration",
					),
				AdultOrientedEnumeration::HealthcareConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						2u32,
						"HealthcareConsideration",
					),
				AdultOrientedEnumeration::NarcoticConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						3u32,
						"NarcoticConsideration",
					),
				AdultOrientedEnumeration::ReducedRelevanceForChildrenConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						4u32,
						"ReducedRelevanceForChildrenConsideration",
					),
				AdultOrientedEnumeration::SexualContentConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						5u32,
						"SexualContentConsideration",
					),
				AdultOrientedEnumeration::TobaccoNicotineConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						6u32,
						"TobaccoNicotineConsideration",
					),
				AdultOrientedEnumeration::UnclassifiedAdultConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						7u32,
						"UnclassifiedAdultConsideration",
					),
				AdultOrientedEnumeration::ViolenceConsideration => serializer
					.serialize_unit_variant(
						"AdultOrientedEnumeration",
						8u32,
						"ViolenceConsideration",
					),
				AdultOrientedEnumeration::WeaponConsideration => serializer.serialize_unit_variant(
					"AdultOrientedEnumeration",
					9u32,
					"WeaponConsideration",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for AdultOrientedEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AlcoholConsideration,
				DangerousGoodConsideration,
				HealthcareConsideration,
				NarcoticConsideration,
				ReducedRelevanceForChildrenConsideration,
				SexualContentConsideration,
				TobaccoNicotineConsideration,
				UnclassifiedAdultConsideration,
				ViolenceConsideration,
				WeaponConsideration,
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
						"AlcoholConsideration" => Ok(Field::AlcoholConsideration),
						"DangerousGoodConsideration" => Ok(Field::DangerousGoodConsideration),
						"HealthcareConsideration" => Ok(Field::HealthcareConsideration),
						"NarcoticConsideration" => Ok(Field::NarcoticConsideration),
						"ReducedRelevanceForChildrenConsideration" => {
							Ok(Field::ReducedRelevanceForChildrenConsideration)
						}
						"SexualContentConsideration" => Ok(Field::SexualContentConsideration),
						"TobaccoNicotineConsideration" => Ok(Field::TobaccoNicotineConsideration),
						"UnclassifiedAdultConsideration" => {
							Ok(Field::UnclassifiedAdultConsideration)
						}
						"ViolenceConsideration" => Ok(Field::ViolenceConsideration),
						"WeaponConsideration" => Ok(Field::WeaponConsideration),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AlcoholConsideration" => Ok(Field::AlcoholConsideration),
						b"DangerousGoodConsideration" => Ok(Field::DangerousGoodConsideration),
						b"HealthcareConsideration" => Ok(Field::HealthcareConsideration),
						b"NarcoticConsideration" => Ok(Field::NarcoticConsideration),
						b"ReducedRelevanceForChildrenConsideration" => {
							Ok(Field::ReducedRelevanceForChildrenConsideration)
						}
						b"SexualContentConsideration" => Ok(Field::SexualContentConsideration),
						b"TobaccoNicotineConsideration" => Ok(Field::TobaccoNicotineConsideration),
						b"UnclassifiedAdultConsideration" => {
							Ok(Field::UnclassifiedAdultConsideration)
						}
						b"ViolenceConsideration" => Ok(Field::ViolenceConsideration),
						b"WeaponConsideration" => Ok(Field::WeaponConsideration),
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
				type Value = AdultOrientedEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema AdultOrientedEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AlcoholConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::AlcoholConsideration)
						}
						(Field::DangerousGoodConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::DangerousGoodConsideration)
						}
						(Field::HealthcareConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::HealthcareConsideration)
						}
						(Field::NarcoticConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::NarcoticConsideration)
						}
						(Field::ReducedRelevanceForChildrenConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::ReducedRelevanceForChildrenConsideration)
						}
						(Field::SexualContentConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::SexualContentConsideration)
						}
						(Field::TobaccoNicotineConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::TobaccoNicotineConsideration)
						}
						(Field::UnclassifiedAdultConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::UnclassifiedAdultConsideration)
						}
						(Field::ViolenceConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::ViolenceConsideration)
						}
						(Field::WeaponConsideration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(AdultOrientedEnumeration::WeaponConsideration)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AlcoholConsideration",
				"DangerousGoodConsideration",
				"HealthcareConsideration",
				"NarcoticConsideration",
				"ReducedRelevanceForChildrenConsideration",
				"SexualContentConsideration",
				"TobaccoNicotineConsideration",
				"UnclassifiedAdultConsideration",
				"ViolenceConsideration",
				"WeaponConsideration",
			];
			deserializer.deserialize_enum("AdultOrientedEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}

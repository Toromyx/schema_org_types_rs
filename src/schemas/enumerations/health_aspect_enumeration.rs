/// <https://schema.org/HealthAspectEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum HealthAspectEnumeration {
	/// <https://schema.org/AllergiesHealthAspect>
	AllergiesHealthAspect,
	/// <https://schema.org/BenefitsHealthAspect>
	BenefitsHealthAspect,
	/// <https://schema.org/CausesHealthAspect>
	CausesHealthAspect,
	/// <https://schema.org/ContagiousnessHealthAspect>
	ContagiousnessHealthAspect,
	/// <https://schema.org/EffectivenessHealthAspect>
	EffectivenessHealthAspect,
	/// <https://schema.org/GettingAccessHealthAspect>
	GettingAccessHealthAspect,
	/// <https://schema.org/HowItWorksHealthAspect>
	HowItWorksHealthAspect,
	/// <https://schema.org/HowOrWhereHealthAspect>
	HowOrWhereHealthAspect,
	/// <https://schema.org/IngredientsHealthAspect>
	IngredientsHealthAspect,
	/// <https://schema.org/LivingWithHealthAspect>
	LivingWithHealthAspect,
	/// <https://schema.org/MayTreatHealthAspect>
	MayTreatHealthAspect,
	/// <https://schema.org/MisconceptionsHealthAspect>
	MisconceptionsHealthAspect,
	/// <https://schema.org/OverviewHealthAspect>
	OverviewHealthAspect,
	/// <https://schema.org/PatientExperienceHealthAspect>
	PatientExperienceHealthAspect,
	/// <https://schema.org/PregnancyHealthAspect>
	PregnancyHealthAspect,
	/// <https://schema.org/PreventionHealthAspect>
	PreventionHealthAspect,
	/// <https://schema.org/PrognosisHealthAspect>
	PrognosisHealthAspect,
	/// <https://schema.org/RelatedTopicsHealthAspect>
	RelatedTopicsHealthAspect,
	/// <https://schema.org/RisksOrComplicationsHealthAspect>
	RisksOrComplicationsHealthAspect,
	/// <https://schema.org/SafetyHealthAspect>
	SafetyHealthAspect,
	/// <https://schema.org/ScreeningHealthAspect>
	ScreeningHealthAspect,
	/// <https://schema.org/SeeDoctorHealthAspect>
	SeeDoctorHealthAspect,
	/// <https://schema.org/SelfCareHealthAspect>
	SelfCareHealthAspect,
	/// <https://schema.org/SideEffectsHealthAspect>
	SideEffectsHealthAspect,
	/// <https://schema.org/StagesHealthAspect>
	StagesHealthAspect,
	/// <https://schema.org/SymptomsHealthAspect>
	SymptomsHealthAspect,
	/// <https://schema.org/TreatmentsHealthAspect>
	TreatmentsHealthAspect,
	/// <https://schema.org/TypesHealthAspect>
	TypesHealthAspect,
	/// <https://schema.org/UsageOrScheduleHealthAspect>
	UsageOrScheduleHealthAspect,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for HealthAspectEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				HealthAspectEnumeration::AllergiesHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						0u32,
						"AllergiesHealthAspect",
					),
				HealthAspectEnumeration::BenefitsHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					1u32,
					"BenefitsHealthAspect",
				),
				HealthAspectEnumeration::CausesHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					2u32,
					"CausesHealthAspect",
				),
				HealthAspectEnumeration::ContagiousnessHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						3u32,
						"ContagiousnessHealthAspect",
					),
				HealthAspectEnumeration::EffectivenessHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						4u32,
						"EffectivenessHealthAspect",
					),
				HealthAspectEnumeration::GettingAccessHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						5u32,
						"GettingAccessHealthAspect",
					),
				HealthAspectEnumeration::HowItWorksHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						6u32,
						"HowItWorksHealthAspect",
					),
				HealthAspectEnumeration::HowOrWhereHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						7u32,
						"HowOrWhereHealthAspect",
					),
				HealthAspectEnumeration::IngredientsHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						8u32,
						"IngredientsHealthAspect",
					),
				HealthAspectEnumeration::LivingWithHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						9u32,
						"LivingWithHealthAspect",
					),
				HealthAspectEnumeration::MayTreatHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					10u32,
					"MayTreatHealthAspect",
				),
				HealthAspectEnumeration::MisconceptionsHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						11u32,
						"MisconceptionsHealthAspect",
					),
				HealthAspectEnumeration::OverviewHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					12u32,
					"OverviewHealthAspect",
				),
				HealthAspectEnumeration::PatientExperienceHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						13u32,
						"PatientExperienceHealthAspect",
					),
				HealthAspectEnumeration::PregnancyHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						14u32,
						"PregnancyHealthAspect",
					),
				HealthAspectEnumeration::PreventionHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						15u32,
						"PreventionHealthAspect",
					),
				HealthAspectEnumeration::PrognosisHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						16u32,
						"PrognosisHealthAspect",
					),
				HealthAspectEnumeration::RelatedTopicsHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						17u32,
						"RelatedTopicsHealthAspect",
					),
				HealthAspectEnumeration::RisksOrComplicationsHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						18u32,
						"RisksOrComplicationsHealthAspect",
					),
				HealthAspectEnumeration::SafetyHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					19u32,
					"SafetyHealthAspect",
				),
				HealthAspectEnumeration::ScreeningHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						20u32,
						"ScreeningHealthAspect",
					),
				HealthAspectEnumeration::SeeDoctorHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						21u32,
						"SeeDoctorHealthAspect",
					),
				HealthAspectEnumeration::SelfCareHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					22u32,
					"SelfCareHealthAspect",
				),
				HealthAspectEnumeration::SideEffectsHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						23u32,
						"SideEffectsHealthAspect",
					),
				HealthAspectEnumeration::StagesHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					24u32,
					"StagesHealthAspect",
				),
				HealthAspectEnumeration::SymptomsHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					25u32,
					"SymptomsHealthAspect",
				),
				HealthAspectEnumeration::TreatmentsHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						26u32,
						"TreatmentsHealthAspect",
					),
				HealthAspectEnumeration::TypesHealthAspect => serializer.serialize_unit_variant(
					"HealthAspectEnumeration",
					27u32,
					"TypesHealthAspect",
				),
				HealthAspectEnumeration::UsageOrScheduleHealthAspect => serializer
					.serialize_unit_variant(
						"HealthAspectEnumeration",
						28u32,
						"UsageOrScheduleHealthAspect",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for HealthAspectEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AllergiesHealthAspect,
				BenefitsHealthAspect,
				CausesHealthAspect,
				ContagiousnessHealthAspect,
				EffectivenessHealthAspect,
				GettingAccessHealthAspect,
				HowItWorksHealthAspect,
				HowOrWhereHealthAspect,
				IngredientsHealthAspect,
				LivingWithHealthAspect,
				MayTreatHealthAspect,
				MisconceptionsHealthAspect,
				OverviewHealthAspect,
				PatientExperienceHealthAspect,
				PregnancyHealthAspect,
				PreventionHealthAspect,
				PrognosisHealthAspect,
				RelatedTopicsHealthAspect,
				RisksOrComplicationsHealthAspect,
				SafetyHealthAspect,
				ScreeningHealthAspect,
				SeeDoctorHealthAspect,
				SelfCareHealthAspect,
				SideEffectsHealthAspect,
				StagesHealthAspect,
				SymptomsHealthAspect,
				TreatmentsHealthAspect,
				TypesHealthAspect,
				UsageOrScheduleHealthAspect,
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
						"AllergiesHealthAspect" => Ok(Field::AllergiesHealthAspect),
						"BenefitsHealthAspect" => Ok(Field::BenefitsHealthAspect),
						"CausesHealthAspect" => Ok(Field::CausesHealthAspect),
						"ContagiousnessHealthAspect" => Ok(Field::ContagiousnessHealthAspect),
						"EffectivenessHealthAspect" => Ok(Field::EffectivenessHealthAspect),
						"GettingAccessHealthAspect" => Ok(Field::GettingAccessHealthAspect),
						"HowItWorksHealthAspect" => Ok(Field::HowItWorksHealthAspect),
						"HowOrWhereHealthAspect" => Ok(Field::HowOrWhereHealthAspect),
						"IngredientsHealthAspect" => Ok(Field::IngredientsHealthAspect),
						"LivingWithHealthAspect" => Ok(Field::LivingWithHealthAspect),
						"MayTreatHealthAspect" => Ok(Field::MayTreatHealthAspect),
						"MisconceptionsHealthAspect" => Ok(Field::MisconceptionsHealthAspect),
						"OverviewHealthAspect" => Ok(Field::OverviewHealthAspect),
						"PatientExperienceHealthAspect" => Ok(Field::PatientExperienceHealthAspect),
						"PregnancyHealthAspect" => Ok(Field::PregnancyHealthAspect),
						"PreventionHealthAspect" => Ok(Field::PreventionHealthAspect),
						"PrognosisHealthAspect" => Ok(Field::PrognosisHealthAspect),
						"RelatedTopicsHealthAspect" => Ok(Field::RelatedTopicsHealthAspect),
						"RisksOrComplicationsHealthAspect" => {
							Ok(Field::RisksOrComplicationsHealthAspect)
						}
						"SafetyHealthAspect" => Ok(Field::SafetyHealthAspect),
						"ScreeningHealthAspect" => Ok(Field::ScreeningHealthAspect),
						"SeeDoctorHealthAspect" => Ok(Field::SeeDoctorHealthAspect),
						"SelfCareHealthAspect" => Ok(Field::SelfCareHealthAspect),
						"SideEffectsHealthAspect" => Ok(Field::SideEffectsHealthAspect),
						"StagesHealthAspect" => Ok(Field::StagesHealthAspect),
						"SymptomsHealthAspect" => Ok(Field::SymptomsHealthAspect),
						"TreatmentsHealthAspect" => Ok(Field::TreatmentsHealthAspect),
						"TypesHealthAspect" => Ok(Field::TypesHealthAspect),
						"UsageOrScheduleHealthAspect" => Ok(Field::UsageOrScheduleHealthAspect),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AllergiesHealthAspect" => Ok(Field::AllergiesHealthAspect),
						b"BenefitsHealthAspect" => Ok(Field::BenefitsHealthAspect),
						b"CausesHealthAspect" => Ok(Field::CausesHealthAspect),
						b"ContagiousnessHealthAspect" => Ok(Field::ContagiousnessHealthAspect),
						b"EffectivenessHealthAspect" => Ok(Field::EffectivenessHealthAspect),
						b"GettingAccessHealthAspect" => Ok(Field::GettingAccessHealthAspect),
						b"HowItWorksHealthAspect" => Ok(Field::HowItWorksHealthAspect),
						b"HowOrWhereHealthAspect" => Ok(Field::HowOrWhereHealthAspect),
						b"IngredientsHealthAspect" => Ok(Field::IngredientsHealthAspect),
						b"LivingWithHealthAspect" => Ok(Field::LivingWithHealthAspect),
						b"MayTreatHealthAspect" => Ok(Field::MayTreatHealthAspect),
						b"MisconceptionsHealthAspect" => Ok(Field::MisconceptionsHealthAspect),
						b"OverviewHealthAspect" => Ok(Field::OverviewHealthAspect),
						b"PatientExperienceHealthAspect" => {
							Ok(Field::PatientExperienceHealthAspect)
						}
						b"PregnancyHealthAspect" => Ok(Field::PregnancyHealthAspect),
						b"PreventionHealthAspect" => Ok(Field::PreventionHealthAspect),
						b"PrognosisHealthAspect" => Ok(Field::PrognosisHealthAspect),
						b"RelatedTopicsHealthAspect" => Ok(Field::RelatedTopicsHealthAspect),
						b"RisksOrComplicationsHealthAspect" => {
							Ok(Field::RisksOrComplicationsHealthAspect)
						}
						b"SafetyHealthAspect" => Ok(Field::SafetyHealthAspect),
						b"ScreeningHealthAspect" => Ok(Field::ScreeningHealthAspect),
						b"SeeDoctorHealthAspect" => Ok(Field::SeeDoctorHealthAspect),
						b"SelfCareHealthAspect" => Ok(Field::SelfCareHealthAspect),
						b"SideEffectsHealthAspect" => Ok(Field::SideEffectsHealthAspect),
						b"StagesHealthAspect" => Ok(Field::StagesHealthAspect),
						b"SymptomsHealthAspect" => Ok(Field::SymptomsHealthAspect),
						b"TreatmentsHealthAspect" => Ok(Field::TreatmentsHealthAspect),
						b"TypesHealthAspect" => Ok(Field::TypesHealthAspect),
						b"UsageOrScheduleHealthAspect" => Ok(Field::UsageOrScheduleHealthAspect),
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
				type Value = HealthAspectEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema HealthAspectEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AllergiesHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::AllergiesHealthAspect)
						}
						(Field::BenefitsHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::BenefitsHealthAspect)
						}
						(Field::CausesHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::CausesHealthAspect)
						}
						(Field::ContagiousnessHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::ContagiousnessHealthAspect)
						}
						(Field::EffectivenessHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::EffectivenessHealthAspect)
						}
						(Field::GettingAccessHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::GettingAccessHealthAspect)
						}
						(Field::HowItWorksHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::HowItWorksHealthAspect)
						}
						(Field::HowOrWhereHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::HowOrWhereHealthAspect)
						}
						(Field::IngredientsHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::IngredientsHealthAspect)
						}
						(Field::LivingWithHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::LivingWithHealthAspect)
						}
						(Field::MayTreatHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::MayTreatHealthAspect)
						}
						(Field::MisconceptionsHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::MisconceptionsHealthAspect)
						}
						(Field::OverviewHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::OverviewHealthAspect)
						}
						(Field::PatientExperienceHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::PatientExperienceHealthAspect)
						}
						(Field::PregnancyHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::PregnancyHealthAspect)
						}
						(Field::PreventionHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::PreventionHealthAspect)
						}
						(Field::PrognosisHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::PrognosisHealthAspect)
						}
						(Field::RelatedTopicsHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::RelatedTopicsHealthAspect)
						}
						(Field::RisksOrComplicationsHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::RisksOrComplicationsHealthAspect)
						}
						(Field::SafetyHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::SafetyHealthAspect)
						}
						(Field::ScreeningHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::ScreeningHealthAspect)
						}
						(Field::SeeDoctorHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::SeeDoctorHealthAspect)
						}
						(Field::SelfCareHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::SelfCareHealthAspect)
						}
						(Field::SideEffectsHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::SideEffectsHealthAspect)
						}
						(Field::StagesHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::StagesHealthAspect)
						}
						(Field::SymptomsHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::SymptomsHealthAspect)
						}
						(Field::TreatmentsHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::TreatmentsHealthAspect)
						}
						(Field::TypesHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::TypesHealthAspect)
						}
						(Field::UsageOrScheduleHealthAspect, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(HealthAspectEnumeration::UsageOrScheduleHealthAspect)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AllergiesHealthAspect",
				"BenefitsHealthAspect",
				"CausesHealthAspect",
				"ContagiousnessHealthAspect",
				"EffectivenessHealthAspect",
				"GettingAccessHealthAspect",
				"HowItWorksHealthAspect",
				"HowOrWhereHealthAspect",
				"IngredientsHealthAspect",
				"LivingWithHealthAspect",
				"MayTreatHealthAspect",
				"MisconceptionsHealthAspect",
				"OverviewHealthAspect",
				"PatientExperienceHealthAspect",
				"PregnancyHealthAspect",
				"PreventionHealthAspect",
				"PrognosisHealthAspect",
				"RelatedTopicsHealthAspect",
				"RisksOrComplicationsHealthAspect",
				"SafetyHealthAspect",
				"ScreeningHealthAspect",
				"SeeDoctorHealthAspect",
				"SelfCareHealthAspect",
				"SideEffectsHealthAspect",
				"StagesHealthAspect",
				"SymptomsHealthAspect",
				"TreatmentsHealthAspect",
				"TypesHealthAspect",
				"UsageOrScheduleHealthAspect",
			];
			deserializer.deserialize_enum("HealthAspectEnumeration", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/MedicalSpecialty>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalSpecialty {
	/// <https://schema.org/Anesthesia>
	Anesthesia,
	/// <https://schema.org/Cardiovascular>
	Cardiovascular,
	/// <https://schema.org/CommunityHealth>
	CommunityHealth,
	/// <https://schema.org/Dentistry>
	Dentistry,
	/// <https://schema.org/Dermatologic>
	#[deprecated = "This schema is superseded by <https://schema.org/Dermatology>."]
	Dermatologic,
	/// <https://schema.org/Dermatology>
	Dermatology,
	/// <https://schema.org/DietNutrition>
	DietNutrition,
	/// <https://schema.org/Emergency>
	Emergency,
	/// <https://schema.org/Endocrine>
	Endocrine,
	/// <https://schema.org/Gastroenterologic>
	Gastroenterologic,
	/// <https://schema.org/Genetic>
	Genetic,
	/// <https://schema.org/Geriatric>
	Geriatric,
	/// <https://schema.org/Gynecologic>
	Gynecologic,
	/// <https://schema.org/Hematologic>
	Hematologic,
	/// <https://schema.org/Infectious>
	Infectious,
	/// <https://schema.org/LaboratoryScience>
	LaboratoryScience,
	/// <https://schema.org/Midwifery>
	Midwifery,
	/// <https://schema.org/Musculoskeletal>
	Musculoskeletal,
	/// <https://schema.org/Neurologic>
	Neurologic,
	/// <https://schema.org/Nursing>
	Nursing,
	/// <https://schema.org/Obstetric>
	Obstetric,
	/// <https://schema.org/Oncologic>
	Oncologic,
	/// <https://schema.org/Optometric>
	Optometric,
	/// <https://schema.org/Otolaryngologic>
	Otolaryngologic,
	/// <https://schema.org/Pathology>
	Pathology,
	/// <https://schema.org/Pediatric>
	Pediatric,
	/// <https://schema.org/PharmacySpecialty>
	PharmacySpecialty,
	/// <https://schema.org/Physiotherapy>
	Physiotherapy,
	/// <https://schema.org/PlasticSurgery>
	PlasticSurgery,
	/// <https://schema.org/Podiatric>
	Podiatric,
	/// <https://schema.org/PrimaryCare>
	PrimaryCare,
	/// <https://schema.org/Psychiatric>
	Psychiatric,
	/// <https://schema.org/PublicHealth>
	PublicHealth,
	/// <https://schema.org/Pulmonary>
	Pulmonary,
	/// <https://schema.org/Radiography>
	Radiography,
	/// <https://schema.org/Renal>
	Renal,
	/// <https://schema.org/RespiratoryTherapy>
	RespiratoryTherapy,
	/// <https://schema.org/Rheumatologic>
	Rheumatologic,
	/// <https://schema.org/SpeechPathology>
	SpeechPathology,
	/// <https://schema.org/Surgical>
	Surgical,
	/// <https://schema.org/Toxicologic>
	Toxicologic,
	/// <https://schema.org/Urologic>
	Urologic,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalSpecialty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalSpecialty::Anesthesia => {
					serializer.serialize_unit_variant("MedicalSpecialty", 0u32, "Anesthesia")
				}
				MedicalSpecialty::Cardiovascular => {
					serializer.serialize_unit_variant("MedicalSpecialty", 1u32, "Cardiovascular")
				}
				MedicalSpecialty::CommunityHealth => {
					serializer.serialize_unit_variant("MedicalSpecialty", 2u32, "CommunityHealth")
				}
				MedicalSpecialty::Dentistry => {
					serializer.serialize_unit_variant("MedicalSpecialty", 3u32, "Dentistry")
				}
				MedicalSpecialty::Dermatologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 4u32, "Dermatologic")
				}
				MedicalSpecialty::Dermatology => {
					serializer.serialize_unit_variant("MedicalSpecialty", 5u32, "Dermatology")
				}
				MedicalSpecialty::DietNutrition => {
					serializer.serialize_unit_variant("MedicalSpecialty", 6u32, "DietNutrition")
				}
				MedicalSpecialty::Emergency => {
					serializer.serialize_unit_variant("MedicalSpecialty", 7u32, "Emergency")
				}
				MedicalSpecialty::Endocrine => {
					serializer.serialize_unit_variant("MedicalSpecialty", 8u32, "Endocrine")
				}
				MedicalSpecialty::Gastroenterologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 9u32, "Gastroenterologic")
				}
				MedicalSpecialty::Genetic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 10u32, "Genetic")
				}
				MedicalSpecialty::Geriatric => {
					serializer.serialize_unit_variant("MedicalSpecialty", 11u32, "Geriatric")
				}
				MedicalSpecialty::Gynecologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 12u32, "Gynecologic")
				}
				MedicalSpecialty::Hematologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 13u32, "Hematologic")
				}
				MedicalSpecialty::Infectious => {
					serializer.serialize_unit_variant("MedicalSpecialty", 14u32, "Infectious")
				}
				MedicalSpecialty::LaboratoryScience => serializer.serialize_unit_variant(
					"MedicalSpecialty",
					15u32,
					"LaboratoryScience",
				),
				MedicalSpecialty::Midwifery => {
					serializer.serialize_unit_variant("MedicalSpecialty", 16u32, "Midwifery")
				}
				MedicalSpecialty::Musculoskeletal => {
					serializer.serialize_unit_variant("MedicalSpecialty", 17u32, "Musculoskeletal")
				}
				MedicalSpecialty::Neurologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 18u32, "Neurologic")
				}
				MedicalSpecialty::Nursing => {
					serializer.serialize_unit_variant("MedicalSpecialty", 19u32, "Nursing")
				}
				MedicalSpecialty::Obstetric => {
					serializer.serialize_unit_variant("MedicalSpecialty", 20u32, "Obstetric")
				}
				MedicalSpecialty::Oncologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 21u32, "Oncologic")
				}
				MedicalSpecialty::Optometric => {
					serializer.serialize_unit_variant("MedicalSpecialty", 22u32, "Optometric")
				}
				MedicalSpecialty::Otolaryngologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 23u32, "Otolaryngologic")
				}
				MedicalSpecialty::Pathology => {
					serializer.serialize_unit_variant("MedicalSpecialty", 24u32, "Pathology")
				}
				MedicalSpecialty::Pediatric => {
					serializer.serialize_unit_variant("MedicalSpecialty", 25u32, "Pediatric")
				}
				MedicalSpecialty::PharmacySpecialty => serializer.serialize_unit_variant(
					"MedicalSpecialty",
					26u32,
					"PharmacySpecialty",
				),
				MedicalSpecialty::Physiotherapy => {
					serializer.serialize_unit_variant("MedicalSpecialty", 27u32, "Physiotherapy")
				}
				MedicalSpecialty::PlasticSurgery => {
					serializer.serialize_unit_variant("MedicalSpecialty", 28u32, "PlasticSurgery")
				}
				MedicalSpecialty::Podiatric => {
					serializer.serialize_unit_variant("MedicalSpecialty", 29u32, "Podiatric")
				}
				MedicalSpecialty::PrimaryCare => {
					serializer.serialize_unit_variant("MedicalSpecialty", 30u32, "PrimaryCare")
				}
				MedicalSpecialty::Psychiatric => {
					serializer.serialize_unit_variant("MedicalSpecialty", 31u32, "Psychiatric")
				}
				MedicalSpecialty::PublicHealth => {
					serializer.serialize_unit_variant("MedicalSpecialty", 32u32, "PublicHealth")
				}
				MedicalSpecialty::Pulmonary => {
					serializer.serialize_unit_variant("MedicalSpecialty", 33u32, "Pulmonary")
				}
				MedicalSpecialty::Radiography => {
					serializer.serialize_unit_variant("MedicalSpecialty", 34u32, "Radiography")
				}
				MedicalSpecialty::Renal => {
					serializer.serialize_unit_variant("MedicalSpecialty", 35u32, "Renal")
				}
				MedicalSpecialty::RespiratoryTherapy => serializer.serialize_unit_variant(
					"MedicalSpecialty",
					36u32,
					"RespiratoryTherapy",
				),
				MedicalSpecialty::Rheumatologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 37u32, "Rheumatologic")
				}
				MedicalSpecialty::SpeechPathology => {
					serializer.serialize_unit_variant("MedicalSpecialty", 38u32, "SpeechPathology")
				}
				MedicalSpecialty::Surgical => {
					serializer.serialize_unit_variant("MedicalSpecialty", 39u32, "Surgical")
				}
				MedicalSpecialty::Toxicologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 40u32, "Toxicologic")
				}
				MedicalSpecialty::Urologic => {
					serializer.serialize_unit_variant("MedicalSpecialty", 41u32, "Urologic")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalSpecialty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Anesthesia,
				Cardiovascular,
				CommunityHealth,
				Dentistry,
				Dermatologic,
				Dermatology,
				DietNutrition,
				Emergency,
				Endocrine,
				Gastroenterologic,
				Genetic,
				Geriatric,
				Gynecologic,
				Hematologic,
				Infectious,
				LaboratoryScience,
				Midwifery,
				Musculoskeletal,
				Neurologic,
				Nursing,
				Obstetric,
				Oncologic,
				Optometric,
				Otolaryngologic,
				Pathology,
				Pediatric,
				PharmacySpecialty,
				Physiotherapy,
				PlasticSurgery,
				Podiatric,
				PrimaryCare,
				Psychiatric,
				PublicHealth,
				Pulmonary,
				Radiography,
				Renal,
				RespiratoryTherapy,
				Rheumatologic,
				SpeechPathology,
				Surgical,
				Toxicologic,
				Urologic,
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
						"Anesthesia" => Ok(Field::Anesthesia),
						"Cardiovascular" => Ok(Field::Cardiovascular),
						"CommunityHealth" => Ok(Field::CommunityHealth),
						"Dentistry" => Ok(Field::Dentistry),
						"Dermatologic" => Ok(Field::Dermatologic),
						"Dermatology" => Ok(Field::Dermatology),
						"DietNutrition" => Ok(Field::DietNutrition),
						"Emergency" => Ok(Field::Emergency),
						"Endocrine" => Ok(Field::Endocrine),
						"Gastroenterologic" => Ok(Field::Gastroenterologic),
						"Genetic" => Ok(Field::Genetic),
						"Geriatric" => Ok(Field::Geriatric),
						"Gynecologic" => Ok(Field::Gynecologic),
						"Hematologic" => Ok(Field::Hematologic),
						"Infectious" => Ok(Field::Infectious),
						"LaboratoryScience" => Ok(Field::LaboratoryScience),
						"Midwifery" => Ok(Field::Midwifery),
						"Musculoskeletal" => Ok(Field::Musculoskeletal),
						"Neurologic" => Ok(Field::Neurologic),
						"Nursing" => Ok(Field::Nursing),
						"Obstetric" => Ok(Field::Obstetric),
						"Oncologic" => Ok(Field::Oncologic),
						"Optometric" => Ok(Field::Optometric),
						"Otolaryngologic" => Ok(Field::Otolaryngologic),
						"Pathology" => Ok(Field::Pathology),
						"Pediatric" => Ok(Field::Pediatric),
						"PharmacySpecialty" => Ok(Field::PharmacySpecialty),
						"Physiotherapy" => Ok(Field::Physiotherapy),
						"PlasticSurgery" => Ok(Field::PlasticSurgery),
						"Podiatric" => Ok(Field::Podiatric),
						"PrimaryCare" => Ok(Field::PrimaryCare),
						"Psychiatric" => Ok(Field::Psychiatric),
						"PublicHealth" => Ok(Field::PublicHealth),
						"Pulmonary" => Ok(Field::Pulmonary),
						"Radiography" => Ok(Field::Radiography),
						"Renal" => Ok(Field::Renal),
						"RespiratoryTherapy" => Ok(Field::RespiratoryTherapy),
						"Rheumatologic" => Ok(Field::Rheumatologic),
						"SpeechPathology" => Ok(Field::SpeechPathology),
						"Surgical" => Ok(Field::Surgical),
						"Toxicologic" => Ok(Field::Toxicologic),
						"Urologic" => Ok(Field::Urologic),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Anesthesia" => Ok(Field::Anesthesia),
						b"Cardiovascular" => Ok(Field::Cardiovascular),
						b"CommunityHealth" => Ok(Field::CommunityHealth),
						b"Dentistry" => Ok(Field::Dentistry),
						b"Dermatologic" => Ok(Field::Dermatologic),
						b"Dermatology" => Ok(Field::Dermatology),
						b"DietNutrition" => Ok(Field::DietNutrition),
						b"Emergency" => Ok(Field::Emergency),
						b"Endocrine" => Ok(Field::Endocrine),
						b"Gastroenterologic" => Ok(Field::Gastroenterologic),
						b"Genetic" => Ok(Field::Genetic),
						b"Geriatric" => Ok(Field::Geriatric),
						b"Gynecologic" => Ok(Field::Gynecologic),
						b"Hematologic" => Ok(Field::Hematologic),
						b"Infectious" => Ok(Field::Infectious),
						b"LaboratoryScience" => Ok(Field::LaboratoryScience),
						b"Midwifery" => Ok(Field::Midwifery),
						b"Musculoskeletal" => Ok(Field::Musculoskeletal),
						b"Neurologic" => Ok(Field::Neurologic),
						b"Nursing" => Ok(Field::Nursing),
						b"Obstetric" => Ok(Field::Obstetric),
						b"Oncologic" => Ok(Field::Oncologic),
						b"Optometric" => Ok(Field::Optometric),
						b"Otolaryngologic" => Ok(Field::Otolaryngologic),
						b"Pathology" => Ok(Field::Pathology),
						b"Pediatric" => Ok(Field::Pediatric),
						b"PharmacySpecialty" => Ok(Field::PharmacySpecialty),
						b"Physiotherapy" => Ok(Field::Physiotherapy),
						b"PlasticSurgery" => Ok(Field::PlasticSurgery),
						b"Podiatric" => Ok(Field::Podiatric),
						b"PrimaryCare" => Ok(Field::PrimaryCare),
						b"Psychiatric" => Ok(Field::Psychiatric),
						b"PublicHealth" => Ok(Field::PublicHealth),
						b"Pulmonary" => Ok(Field::Pulmonary),
						b"Radiography" => Ok(Field::Radiography),
						b"Renal" => Ok(Field::Renal),
						b"RespiratoryTherapy" => Ok(Field::RespiratoryTherapy),
						b"Rheumatologic" => Ok(Field::Rheumatologic),
						b"SpeechPathology" => Ok(Field::SpeechPathology),
						b"Surgical" => Ok(Field::Surgical),
						b"Toxicologic" => Ok(Field::Toxicologic),
						b"Urologic" => Ok(Field::Urologic),
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
				type Value = MedicalSpecialty;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalSpecialty")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Anesthesia, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Anesthesia)
						}
						(Field::Cardiovascular, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Cardiovascular)
						}
						(Field::CommunityHealth, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::CommunityHealth)
						}
						(Field::Dentistry, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Dentistry)
						}
						(Field::Dermatologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Dermatologic)
						}
						(Field::Dermatology, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Dermatology)
						}
						(Field::DietNutrition, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::DietNutrition)
						}
						(Field::Emergency, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Emergency)
						}
						(Field::Endocrine, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Endocrine)
						}
						(Field::Gastroenterologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Gastroenterologic)
						}
						(Field::Genetic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Genetic)
						}
						(Field::Geriatric, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Geriatric)
						}
						(Field::Gynecologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Gynecologic)
						}
						(Field::Hematologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Hematologic)
						}
						(Field::Infectious, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Infectious)
						}
						(Field::LaboratoryScience, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::LaboratoryScience)
						}
						(Field::Midwifery, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Midwifery)
						}
						(Field::Musculoskeletal, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Musculoskeletal)
						}
						(Field::Neurologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Neurologic)
						}
						(Field::Nursing, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Nursing)
						}
						(Field::Obstetric, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Obstetric)
						}
						(Field::Oncologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Oncologic)
						}
						(Field::Optometric, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Optometric)
						}
						(Field::Otolaryngologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Otolaryngologic)
						}
						(Field::Pathology, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Pathology)
						}
						(Field::Pediatric, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Pediatric)
						}
						(Field::PharmacySpecialty, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::PharmacySpecialty)
						}
						(Field::Physiotherapy, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Physiotherapy)
						}
						(Field::PlasticSurgery, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::PlasticSurgery)
						}
						(Field::Podiatric, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Podiatric)
						}
						(Field::PrimaryCare, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::PrimaryCare)
						}
						(Field::Psychiatric, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Psychiatric)
						}
						(Field::PublicHealth, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::PublicHealth)
						}
						(Field::Pulmonary, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Pulmonary)
						}
						(Field::Radiography, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Radiography)
						}
						(Field::Renal, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Renal)
						}
						(Field::RespiratoryTherapy, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::RespiratoryTherapy)
						}
						(Field::Rheumatologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Rheumatologic)
						}
						(Field::SpeechPathology, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::SpeechPathology)
						}
						(Field::Surgical, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Surgical)
						}
						(Field::Toxicologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Toxicologic)
						}
						(Field::Urologic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalSpecialty::Urologic)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"Anesthesia",
				"Cardiovascular",
				"CommunityHealth",
				"Dentistry",
				"Dermatologic",
				"Dermatology",
				"DietNutrition",
				"Emergency",
				"Endocrine",
				"Gastroenterologic",
				"Genetic",
				"Geriatric",
				"Gynecologic",
				"Hematologic",
				"Infectious",
				"LaboratoryScience",
				"Midwifery",
				"Musculoskeletal",
				"Neurologic",
				"Nursing",
				"Obstetric",
				"Oncologic",
				"Optometric",
				"Otolaryngologic",
				"Pathology",
				"Pediatric",
				"PharmacySpecialty",
				"Physiotherapy",
				"PlasticSurgery",
				"Podiatric",
				"PrimaryCare",
				"Psychiatric",
				"PublicHealth",
				"Pulmonary",
				"Radiography",
				"Renal",
				"RespiratoryTherapy",
				"Rheumatologic",
				"SpeechPathology",
				"Surgical",
				"Toxicologic",
				"Urologic",
			];
			deserializer.deserialize_enum("MedicalSpecialty", VARIANTS, EnumerationVisitor)
		}
	}
}

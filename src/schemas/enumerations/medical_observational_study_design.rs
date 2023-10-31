/// <https://schema.org/MedicalObservationalStudyDesign>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalObservationalStudyDesign {
	/// <https://schema.org/CaseSeries>
	CaseSeries,
	/// <https://schema.org/CohortStudy>
	CohortStudy,
	/// <https://schema.org/CrossSectional>
	CrossSectional,
	/// <https://schema.org/Longitudinal>
	Longitudinal,
	/// <https://schema.org/Observational>
	Observational,
	/// <https://schema.org/Registry>
	Registry,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalObservationalStudyDesign {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalObservationalStudyDesign::CaseSeries => serializer.serialize_unit_variant(
					"MedicalObservationalStudyDesign",
					0u32,
					"CaseSeries",
				),
				MedicalObservationalStudyDesign::CohortStudy => serializer.serialize_unit_variant(
					"MedicalObservationalStudyDesign",
					1u32,
					"CohortStudy",
				),
				MedicalObservationalStudyDesign::CrossSectional => serializer
					.serialize_unit_variant(
						"MedicalObservationalStudyDesign",
						2u32,
						"CrossSectional",
					),
				MedicalObservationalStudyDesign::Longitudinal => serializer.serialize_unit_variant(
					"MedicalObservationalStudyDesign",
					3u32,
					"Longitudinal",
				),
				MedicalObservationalStudyDesign::Observational => serializer
					.serialize_unit_variant(
						"MedicalObservationalStudyDesign",
						4u32,
						"Observational",
					),
				MedicalObservationalStudyDesign::Registry => serializer.serialize_unit_variant(
					"MedicalObservationalStudyDesign",
					5u32,
					"Registry",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalObservationalStudyDesign {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				CaseSeries,
				CohortStudy,
				CrossSectional,
				Longitudinal,
				Observational,
				Registry,
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
						"CaseSeries" => Ok(Field::CaseSeries),
						"CohortStudy" => Ok(Field::CohortStudy),
						"CrossSectional" => Ok(Field::CrossSectional),
						"Longitudinal" => Ok(Field::Longitudinal),
						"Observational" => Ok(Field::Observational),
						"Registry" => Ok(Field::Registry),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"CaseSeries" => Ok(Field::CaseSeries),
						b"CohortStudy" => Ok(Field::CohortStudy),
						b"CrossSectional" => Ok(Field::CrossSectional),
						b"Longitudinal" => Ok(Field::Longitudinal),
						b"Observational" => Ok(Field::Observational),
						b"Registry" => Ok(Field::Registry),
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
				type Value = MedicalObservationalStudyDesign;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalObservationalStudyDesign")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::CaseSeries, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalObservationalStudyDesign::CaseSeries)
						}
						(Field::CohortStudy, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalObservationalStudyDesign::CohortStudy)
						}
						(Field::CrossSectional, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalObservationalStudyDesign::CrossSectional)
						}
						(Field::Longitudinal, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalObservationalStudyDesign::Longitudinal)
						}
						(Field::Observational, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalObservationalStudyDesign::Observational)
						}
						(Field::Registry, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalObservationalStudyDesign::Registry)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"CaseSeries",
				"CohortStudy",
				"CrossSectional",
				"Longitudinal",
				"Observational",
				"Registry",
			];
			deserializer.deserialize_enum(
				"MedicalObservationalStudyDesign",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

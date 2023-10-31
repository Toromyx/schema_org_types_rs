/// <https://schema.org/MedicalTrialDesign>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalTrialDesign {
	/// <https://schema.org/DoubleBlindedTrial>
	DoubleBlindedTrial,
	/// <https://schema.org/InternationalTrial>
	InternationalTrial,
	/// <https://schema.org/MultiCenterTrial>
	MultiCenterTrial,
	/// <https://schema.org/OpenTrial>
	OpenTrial,
	/// <https://schema.org/PlaceboControlledTrial>
	PlaceboControlledTrial,
	/// <https://schema.org/RandomizedTrial>
	RandomizedTrial,
	/// <https://schema.org/SingleBlindedTrial>
	SingleBlindedTrial,
	/// <https://schema.org/SingleCenterTrial>
	SingleCenterTrial,
	/// <https://schema.org/TripleBlindedTrial>
	TripleBlindedTrial,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalTrialDesign {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalTrialDesign::DoubleBlindedTrial => serializer.serialize_unit_variant(
					"MedicalTrialDesign",
					0u32,
					"DoubleBlindedTrial",
				),
				MedicalTrialDesign::InternationalTrial => serializer.serialize_unit_variant(
					"MedicalTrialDesign",
					1u32,
					"InternationalTrial",
				),
				MedicalTrialDesign::MultiCenterTrial => serializer.serialize_unit_variant(
					"MedicalTrialDesign",
					2u32,
					"MultiCenterTrial",
				),
				MedicalTrialDesign::OpenTrial => {
					serializer.serialize_unit_variant("MedicalTrialDesign", 3u32, "OpenTrial")
				}
				MedicalTrialDesign::PlaceboControlledTrial => serializer.serialize_unit_variant(
					"MedicalTrialDesign",
					4u32,
					"PlaceboControlledTrial",
				),
				MedicalTrialDesign::RandomizedTrial => {
					serializer.serialize_unit_variant("MedicalTrialDesign", 5u32, "RandomizedTrial")
				}
				MedicalTrialDesign::SingleBlindedTrial => serializer.serialize_unit_variant(
					"MedicalTrialDesign",
					6u32,
					"SingleBlindedTrial",
				),
				MedicalTrialDesign::SingleCenterTrial => serializer.serialize_unit_variant(
					"MedicalTrialDesign",
					7u32,
					"SingleCenterTrial",
				),
				MedicalTrialDesign::TripleBlindedTrial => serializer.serialize_unit_variant(
					"MedicalTrialDesign",
					8u32,
					"TripleBlindedTrial",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalTrialDesign {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				DoubleBlindedTrial,
				InternationalTrial,
				MultiCenterTrial,
				OpenTrial,
				PlaceboControlledTrial,
				RandomizedTrial,
				SingleBlindedTrial,
				SingleCenterTrial,
				TripleBlindedTrial,
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
						"DoubleBlindedTrial" => Ok(Field::DoubleBlindedTrial),
						"InternationalTrial" => Ok(Field::InternationalTrial),
						"MultiCenterTrial" => Ok(Field::MultiCenterTrial),
						"OpenTrial" => Ok(Field::OpenTrial),
						"PlaceboControlledTrial" => Ok(Field::PlaceboControlledTrial),
						"RandomizedTrial" => Ok(Field::RandomizedTrial),
						"SingleBlindedTrial" => Ok(Field::SingleBlindedTrial),
						"SingleCenterTrial" => Ok(Field::SingleCenterTrial),
						"TripleBlindedTrial" => Ok(Field::TripleBlindedTrial),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"DoubleBlindedTrial" => Ok(Field::DoubleBlindedTrial),
						b"InternationalTrial" => Ok(Field::InternationalTrial),
						b"MultiCenterTrial" => Ok(Field::MultiCenterTrial),
						b"OpenTrial" => Ok(Field::OpenTrial),
						b"PlaceboControlledTrial" => Ok(Field::PlaceboControlledTrial),
						b"RandomizedTrial" => Ok(Field::RandomizedTrial),
						b"SingleBlindedTrial" => Ok(Field::SingleBlindedTrial),
						b"SingleCenterTrial" => Ok(Field::SingleCenterTrial),
						b"TripleBlindedTrial" => Ok(Field::TripleBlindedTrial),
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
				type Value = MedicalTrialDesign;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalTrialDesign")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::DoubleBlindedTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::DoubleBlindedTrial)
						}
						(Field::InternationalTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::InternationalTrial)
						}
						(Field::MultiCenterTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::MultiCenterTrial)
						}
						(Field::OpenTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::OpenTrial)
						}
						(Field::PlaceboControlledTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::PlaceboControlledTrial)
						}
						(Field::RandomizedTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::RandomizedTrial)
						}
						(Field::SingleBlindedTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::SingleBlindedTrial)
						}
						(Field::SingleCenterTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::SingleCenterTrial)
						}
						(Field::TripleBlindedTrial, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalTrialDesign::TripleBlindedTrial)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"DoubleBlindedTrial",
				"InternationalTrial",
				"MultiCenterTrial",
				"OpenTrial",
				"PlaceboControlledTrial",
				"RandomizedTrial",
				"SingleBlindedTrial",
				"SingleCenterTrial",
				"TripleBlindedTrial",
			];
			deserializer.deserialize_enum("MedicalTrialDesign", VARIANTS, EnumerationVisitor)
		}
	}
}

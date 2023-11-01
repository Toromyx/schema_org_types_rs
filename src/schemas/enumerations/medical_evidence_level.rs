/// <https://schema.org/MedicalEvidenceLevel>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalEvidenceLevel {
	/// <https://schema.org/EvidenceLevelA>
	EvidenceLevelA,
	/// <https://schema.org/EvidenceLevelB>
	EvidenceLevelB,
	/// <https://schema.org/EvidenceLevelC>
	EvidenceLevelC,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalEvidenceLevel {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalEvidenceLevel::EvidenceLevelA => serializer.serialize_unit_variant(
					"MedicalEvidenceLevel",
					0u32,
					"EvidenceLevelA",
				),
				MedicalEvidenceLevel::EvidenceLevelB => serializer.serialize_unit_variant(
					"MedicalEvidenceLevel",
					1u32,
					"EvidenceLevelB",
				),
				MedicalEvidenceLevel::EvidenceLevelC => serializer.serialize_unit_variant(
					"MedicalEvidenceLevel",
					2u32,
					"EvidenceLevelC",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalEvidenceLevel {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				EvidenceLevelA,
				EvidenceLevelB,
				EvidenceLevelC,
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
						"EvidenceLevelA" => Ok(Field::EvidenceLevelA),
						"EvidenceLevelB" => Ok(Field::EvidenceLevelB),
						"EvidenceLevelC" => Ok(Field::EvidenceLevelC),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"EvidenceLevelA" => Ok(Field::EvidenceLevelA),
						b"EvidenceLevelB" => Ok(Field::EvidenceLevelB),
						b"EvidenceLevelC" => Ok(Field::EvidenceLevelC),
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
				type Value = MedicalEvidenceLevel;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalEvidenceLevel")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::EvidenceLevelA, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalEvidenceLevel::EvidenceLevelA)
						}
						(Field::EvidenceLevelB, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalEvidenceLevel::EvidenceLevelB)
						}
						(Field::EvidenceLevelC, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalEvidenceLevel::EvidenceLevelC)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["EvidenceLevelA", "EvidenceLevelB", "EvidenceLevelC"];
			deserializer.deserialize_enum("MedicalEvidenceLevel", VARIANTS, EnumerationVisitor)
		}
	}
}

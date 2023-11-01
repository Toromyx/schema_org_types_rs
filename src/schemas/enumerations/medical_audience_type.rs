/// <https://schema.org/MedicalAudienceType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalAudienceType {
	/// <https://schema.org/Clinician>
	Clinician,
	/// <https://schema.org/MedicalResearcher>
	MedicalResearcher,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalAudienceType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalAudienceType::Clinician => {
					serializer.serialize_unit_variant("MedicalAudienceType", 0u32, "Clinician")
				}
				MedicalAudienceType::MedicalResearcher => serializer.serialize_unit_variant(
					"MedicalAudienceType",
					1u32,
					"MedicalResearcher",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalAudienceType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Clinician,
				MedicalResearcher,
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
						"Clinician" => Ok(Field::Clinician),
						"MedicalResearcher" => Ok(Field::MedicalResearcher),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Clinician" => Ok(Field::Clinician),
						b"MedicalResearcher" => Ok(Field::MedicalResearcher),
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
				type Value = MedicalAudienceType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalAudienceType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Clinician, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalAudienceType::Clinician)
						}
						(Field::MedicalResearcher, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalAudienceType::MedicalResearcher)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["Clinician", "MedicalResearcher"];
			deserializer.deserialize_enum("MedicalAudienceType", VARIANTS, EnumerationVisitor)
		}
	}
}

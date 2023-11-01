/// <https://schema.org/MedicalStudyStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalStudyStatus {
	/// <https://schema.org/ActiveNotRecruiting>
	ActiveNotRecruiting,
	/// <https://schema.org/Completed>
	Completed,
	/// <https://schema.org/EnrollingByInvitation>
	EnrollingByInvitation,
	/// <https://schema.org/NotYetRecruiting>
	NotYetRecruiting,
	/// <https://schema.org/Recruiting>
	Recruiting,
	/// <https://schema.org/ResultsAvailable>
	ResultsAvailable,
	/// <https://schema.org/ResultsNotAvailable>
	ResultsNotAvailable,
	/// <https://schema.org/Suspended>
	Suspended,
	/// <https://schema.org/Terminated>
	Terminated,
	/// <https://schema.org/Withdrawn>
	Withdrawn,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalStudyStatus {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalStudyStatus::ActiveNotRecruiting => serializer.serialize_unit_variant(
					"MedicalStudyStatus",
					0u32,
					"ActiveNotRecruiting",
				),
				MedicalStudyStatus::Completed => {
					serializer.serialize_unit_variant("MedicalStudyStatus", 1u32, "Completed")
				}
				MedicalStudyStatus::EnrollingByInvitation => serializer.serialize_unit_variant(
					"MedicalStudyStatus",
					2u32,
					"EnrollingByInvitation",
				),
				MedicalStudyStatus::NotYetRecruiting => serializer.serialize_unit_variant(
					"MedicalStudyStatus",
					3u32,
					"NotYetRecruiting",
				),
				MedicalStudyStatus::Recruiting => {
					serializer.serialize_unit_variant("MedicalStudyStatus", 4u32, "Recruiting")
				}
				MedicalStudyStatus::ResultsAvailable => serializer.serialize_unit_variant(
					"MedicalStudyStatus",
					5u32,
					"ResultsAvailable",
				),
				MedicalStudyStatus::ResultsNotAvailable => serializer.serialize_unit_variant(
					"MedicalStudyStatus",
					6u32,
					"ResultsNotAvailable",
				),
				MedicalStudyStatus::Suspended => {
					serializer.serialize_unit_variant("MedicalStudyStatus", 7u32, "Suspended")
				}
				MedicalStudyStatus::Terminated => {
					serializer.serialize_unit_variant("MedicalStudyStatus", 8u32, "Terminated")
				}
				MedicalStudyStatus::Withdrawn => {
					serializer.serialize_unit_variant("MedicalStudyStatus", 9u32, "Withdrawn")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalStudyStatus {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ActiveNotRecruiting,
				Completed,
				EnrollingByInvitation,
				NotYetRecruiting,
				Recruiting,
				ResultsAvailable,
				ResultsNotAvailable,
				Suspended,
				Terminated,
				Withdrawn,
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
						"ActiveNotRecruiting" => Ok(Field::ActiveNotRecruiting),
						"Completed" => Ok(Field::Completed),
						"EnrollingByInvitation" => Ok(Field::EnrollingByInvitation),
						"NotYetRecruiting" => Ok(Field::NotYetRecruiting),
						"Recruiting" => Ok(Field::Recruiting),
						"ResultsAvailable" => Ok(Field::ResultsAvailable),
						"ResultsNotAvailable" => Ok(Field::ResultsNotAvailable),
						"Suspended" => Ok(Field::Suspended),
						"Terminated" => Ok(Field::Terminated),
						"Withdrawn" => Ok(Field::Withdrawn),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ActiveNotRecruiting" => Ok(Field::ActiveNotRecruiting),
						b"Completed" => Ok(Field::Completed),
						b"EnrollingByInvitation" => Ok(Field::EnrollingByInvitation),
						b"NotYetRecruiting" => Ok(Field::NotYetRecruiting),
						b"Recruiting" => Ok(Field::Recruiting),
						b"ResultsAvailable" => Ok(Field::ResultsAvailable),
						b"ResultsNotAvailable" => Ok(Field::ResultsNotAvailable),
						b"Suspended" => Ok(Field::Suspended),
						b"Terminated" => Ok(Field::Terminated),
						b"Withdrawn" => Ok(Field::Withdrawn),
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
				type Value = MedicalStudyStatus;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalStudyStatus")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ActiveNotRecruiting, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::ActiveNotRecruiting)
						}
						(Field::Completed, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::Completed)
						}
						(Field::EnrollingByInvitation, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::EnrollingByInvitation)
						}
						(Field::NotYetRecruiting, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::NotYetRecruiting)
						}
						(Field::Recruiting, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::Recruiting)
						}
						(Field::ResultsAvailable, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::ResultsAvailable)
						}
						(Field::ResultsNotAvailable, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::ResultsNotAvailable)
						}
						(Field::Suspended, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::Suspended)
						}
						(Field::Terminated, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::Terminated)
						}
						(Field::Withdrawn, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalStudyStatus::Withdrawn)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ActiveNotRecruiting",
				"Completed",
				"EnrollingByInvitation",
				"NotYetRecruiting",
				"Recruiting",
				"ResultsAvailable",
				"ResultsNotAvailable",
				"Suspended",
				"Terminated",
				"Withdrawn",
			];
			deserializer.deserialize_enum("MedicalStudyStatus", VARIANTS, EnumerationVisitor)
		}
	}
}

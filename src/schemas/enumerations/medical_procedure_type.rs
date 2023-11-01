/// <https://schema.org/MedicalProcedureType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalProcedureType {
	/// <https://schema.org/NoninvasiveProcedure>
	NoninvasiveProcedure,
	/// <https://schema.org/PercutaneousProcedure>
	PercutaneousProcedure,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalProcedureType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalProcedureType::NoninvasiveProcedure => serializer.serialize_unit_variant(
					"MedicalProcedureType",
					0u32,
					"NoninvasiveProcedure",
				),
				MedicalProcedureType::PercutaneousProcedure => serializer.serialize_unit_variant(
					"MedicalProcedureType",
					1u32,
					"PercutaneousProcedure",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalProcedureType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				NoninvasiveProcedure,
				PercutaneousProcedure,
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
						"NoninvasiveProcedure" => Ok(Field::NoninvasiveProcedure),
						"PercutaneousProcedure" => Ok(Field::PercutaneousProcedure),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"NoninvasiveProcedure" => Ok(Field::NoninvasiveProcedure),
						b"PercutaneousProcedure" => Ok(Field::PercutaneousProcedure),
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
				type Value = MedicalProcedureType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalProcedureType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::NoninvasiveProcedure, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalProcedureType::NoninvasiveProcedure)
						}
						(Field::PercutaneousProcedure, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalProcedureType::PercutaneousProcedure)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["NoninvasiveProcedure", "PercutaneousProcedure"];
			deserializer.deserialize_enum("MedicalProcedureType", VARIANTS, EnumerationVisitor)
		}
	}
}

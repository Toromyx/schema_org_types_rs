/// <https://schema.org/DrugPrescriptionStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DrugPrescriptionStatus {
	/// <https://schema.org/OTC>
	Otc,
	/// <https://schema.org/PrescriptionOnly>
	PrescriptionOnly,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DrugPrescriptionStatus {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DrugPrescriptionStatus::Otc => {
					serializer.serialize_unit_variant("DrugPrescriptionStatus", 0u32, "Otc")
				}
				DrugPrescriptionStatus::PrescriptionOnly => serializer.serialize_unit_variant(
					"DrugPrescriptionStatus",
					1u32,
					"PrescriptionOnly",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for DrugPrescriptionStatus {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Otc,
				PrescriptionOnly,
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
						"Otc" => Ok(Field::Otc),
						"PrescriptionOnly" => Ok(Field::PrescriptionOnly),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Otc" => Ok(Field::Otc),
						b"PrescriptionOnly" => Ok(Field::PrescriptionOnly),
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
				type Value = DrugPrescriptionStatus;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DrugPrescriptionStatus")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Otc, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugPrescriptionStatus::Otc)
						}
						(Field::PrescriptionOnly, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugPrescriptionStatus::PrescriptionOnly)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["Otc", "PrescriptionOnly"];
			deserializer.deserialize_enum("DrugPrescriptionStatus", VARIANTS, EnumerationVisitor)
		}
	}
}

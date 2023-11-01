/// <https://schema.org/MedicalDevicePurpose>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalDevicePurpose {
	/// <https://schema.org/Diagnostic>
	Diagnostic,
	/// <https://schema.org/Therapeutic>
	Therapeutic,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalDevicePurpose {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalDevicePurpose::Diagnostic => {
					serializer.serialize_unit_variant("MedicalDevicePurpose", 0u32, "Diagnostic")
				}
				MedicalDevicePurpose::Therapeutic => {
					serializer.serialize_unit_variant("MedicalDevicePurpose", 1u32, "Therapeutic")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalDevicePurpose {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Diagnostic,
				Therapeutic,
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
						"Diagnostic" => Ok(Field::Diagnostic),
						"Therapeutic" => Ok(Field::Therapeutic),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Diagnostic" => Ok(Field::Diagnostic),
						b"Therapeutic" => Ok(Field::Therapeutic),
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
				type Value = MedicalDevicePurpose;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalDevicePurpose")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Diagnostic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalDevicePurpose::Diagnostic)
						}
						(Field::Therapeutic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalDevicePurpose::Therapeutic)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["Diagnostic", "Therapeutic"];
			deserializer.deserialize_enum("MedicalDevicePurpose", VARIANTS, EnumerationVisitor)
		}
	}
}

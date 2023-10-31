/// <https://schema.org/MedicalImagingTechnique>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalImagingTechnique {
	/// <https://schema.org/CT>
	Ct,
	/// <https://schema.org/MRI>
	Mri,
	/// <https://schema.org/PET>
	Pet,
	/// <https://schema.org/Radiography>
	Radiography,
	/// <https://schema.org/Ultrasound>
	Ultrasound,
	/// <https://schema.org/XRay>
	XRay,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalImagingTechnique {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalImagingTechnique::Ct => {
					serializer.serialize_unit_variant("MedicalImagingTechnique", 0u32, "Ct")
				}
				MedicalImagingTechnique::Mri => {
					serializer.serialize_unit_variant("MedicalImagingTechnique", 1u32, "Mri")
				}
				MedicalImagingTechnique::Pet => {
					serializer.serialize_unit_variant("MedicalImagingTechnique", 2u32, "Pet")
				}
				MedicalImagingTechnique::Radiography => serializer.serialize_unit_variant(
					"MedicalImagingTechnique",
					3u32,
					"Radiography",
				),
				MedicalImagingTechnique::Ultrasound => {
					serializer.serialize_unit_variant("MedicalImagingTechnique", 4u32, "Ultrasound")
				}
				MedicalImagingTechnique::XRay => {
					serializer.serialize_unit_variant("MedicalImagingTechnique", 5u32, "XRay")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalImagingTechnique {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Ct,
				Mri,
				Pet,
				Radiography,
				Ultrasound,
				XRay,
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
						"Ct" => Ok(Field::Ct),
						"Mri" => Ok(Field::Mri),
						"Pet" => Ok(Field::Pet),
						"Radiography" => Ok(Field::Radiography),
						"Ultrasound" => Ok(Field::Ultrasound),
						"XRay" => Ok(Field::XRay),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Ct" => Ok(Field::Ct),
						b"Mri" => Ok(Field::Mri),
						b"Pet" => Ok(Field::Pet),
						b"Radiography" => Ok(Field::Radiography),
						b"Ultrasound" => Ok(Field::Ultrasound),
						b"XRay" => Ok(Field::XRay),
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
				type Value = MedicalImagingTechnique;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicalImagingTechnique")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Ct, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalImagingTechnique::Ct)
						}
						(Field::Mri, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalImagingTechnique::Mri)
						}
						(Field::Pet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalImagingTechnique::Pet)
						}
						(Field::Radiography, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalImagingTechnique::Radiography)
						}
						(Field::Ultrasound, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalImagingTechnique::Ultrasound)
						}
						(Field::XRay, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicalImagingTechnique::XRay)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["Ct", "Mri", "Pet", "Radiography", "Ultrasound", "XRay"];
			deserializer.deserialize_enum("MedicalImagingTechnique", VARIANTS, EnumerationVisitor)
		}
	}
}

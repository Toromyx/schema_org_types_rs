/// <https://schema.org/PhysicalExam>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PhysicalExam {
	/// <https://schema.org/Abdomen>
	Abdomen,
	/// <https://schema.org/Appearance>
	Appearance,
	/// <https://schema.org/CardiovascularExam>
	CardiovascularExam,
	/// <https://schema.org/Ear>
	Ear,
	/// <https://schema.org/Eye>
	Eye,
	/// <https://schema.org/Genitourinary>
	Genitourinary,
	/// <https://schema.org/Head>
	Head,
	/// <https://schema.org/Lung>
	Lung,
	/// <https://schema.org/MusculoskeletalExam>
	MusculoskeletalExam,
	/// <https://schema.org/Neck>
	Neck,
	/// <https://schema.org/Neuro>
	Neuro,
	/// <https://schema.org/Nose>
	Nose,
	/// <https://schema.org/Skin>
	Skin,
	/// <https://schema.org/Throat>
	Throat,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for PhysicalExam {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PhysicalExam::Abdomen => {
					serializer.serialize_unit_variant("PhysicalExam", 0u32, "Abdomen")
				}
				PhysicalExam::Appearance => {
					serializer.serialize_unit_variant("PhysicalExam", 1u32, "Appearance")
				}
				PhysicalExam::CardiovascularExam => {
					serializer.serialize_unit_variant("PhysicalExam", 2u32, "CardiovascularExam")
				}
				PhysicalExam::Ear => serializer.serialize_unit_variant("PhysicalExam", 3u32, "Ear"),
				PhysicalExam::Eye => serializer.serialize_unit_variant("PhysicalExam", 4u32, "Eye"),
				PhysicalExam::Genitourinary => {
					serializer.serialize_unit_variant("PhysicalExam", 5u32, "Genitourinary")
				}
				PhysicalExam::Head => {
					serializer.serialize_unit_variant("PhysicalExam", 6u32, "Head")
				}
				PhysicalExam::Lung => {
					serializer.serialize_unit_variant("PhysicalExam", 7u32, "Lung")
				}
				PhysicalExam::MusculoskeletalExam => {
					serializer.serialize_unit_variant("PhysicalExam", 8u32, "MusculoskeletalExam")
				}
				PhysicalExam::Neck => {
					serializer.serialize_unit_variant("PhysicalExam", 9u32, "Neck")
				}
				PhysicalExam::Neuro => {
					serializer.serialize_unit_variant("PhysicalExam", 10u32, "Neuro")
				}
				PhysicalExam::Nose => {
					serializer.serialize_unit_variant("PhysicalExam", 11u32, "Nose")
				}
				PhysicalExam::Skin => {
					serializer.serialize_unit_variant("PhysicalExam", 12u32, "Skin")
				}
				PhysicalExam::Throat => {
					serializer.serialize_unit_variant("PhysicalExam", 13u32, "Throat")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for PhysicalExam {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Abdomen,
				Appearance,
				CardiovascularExam,
				Ear,
				Eye,
				Genitourinary,
				Head,
				Lung,
				MusculoskeletalExam,
				Neck,
				Neuro,
				Nose,
				Skin,
				Throat,
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
						"Abdomen" => Ok(Field::Abdomen),
						"Appearance" => Ok(Field::Appearance),
						"CardiovascularExam" => Ok(Field::CardiovascularExam),
						"Ear" => Ok(Field::Ear),
						"Eye" => Ok(Field::Eye),
						"Genitourinary" => Ok(Field::Genitourinary),
						"Head" => Ok(Field::Head),
						"Lung" => Ok(Field::Lung),
						"MusculoskeletalExam" => Ok(Field::MusculoskeletalExam),
						"Neck" => Ok(Field::Neck),
						"Neuro" => Ok(Field::Neuro),
						"Nose" => Ok(Field::Nose),
						"Skin" => Ok(Field::Skin),
						"Throat" => Ok(Field::Throat),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Abdomen" => Ok(Field::Abdomen),
						b"Appearance" => Ok(Field::Appearance),
						b"CardiovascularExam" => Ok(Field::CardiovascularExam),
						b"Ear" => Ok(Field::Ear),
						b"Eye" => Ok(Field::Eye),
						b"Genitourinary" => Ok(Field::Genitourinary),
						b"Head" => Ok(Field::Head),
						b"Lung" => Ok(Field::Lung),
						b"MusculoskeletalExam" => Ok(Field::MusculoskeletalExam),
						b"Neck" => Ok(Field::Neck),
						b"Neuro" => Ok(Field::Neuro),
						b"Nose" => Ok(Field::Nose),
						b"Skin" => Ok(Field::Skin),
						b"Throat" => Ok(Field::Throat),
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
				type Value = PhysicalExam;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema PhysicalExam")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Abdomen, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Abdomen)
						}
						(Field::Appearance, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Appearance)
						}
						(Field::CardiovascularExam, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::CardiovascularExam)
						}
						(Field::Ear, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Ear)
						}
						(Field::Eye, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Eye)
						}
						(Field::Genitourinary, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Genitourinary)
						}
						(Field::Head, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Head)
						}
						(Field::Lung, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Lung)
						}
						(Field::MusculoskeletalExam, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::MusculoskeletalExam)
						}
						(Field::Neck, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Neck)
						}
						(Field::Neuro, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Neuro)
						}
						(Field::Nose, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Nose)
						}
						(Field::Skin, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Skin)
						}
						(Field::Throat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalExam::Throat)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"Abdomen",
				"Appearance",
				"CardiovascularExam",
				"Ear",
				"Eye",
				"Genitourinary",
				"Head",
				"Lung",
				"MusculoskeletalExam",
				"Neck",
				"Neuro",
				"Nose",
				"Skin",
				"Throat",
			];
			deserializer.deserialize_enum("PhysicalExam", VARIANTS, EnumerationVisitor)
		}
	}
}

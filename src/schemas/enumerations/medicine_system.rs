/// <https://schema.org/MedicineSystem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicineSystem {
	/// <https://schema.org/Ayurvedic>
	Ayurvedic,
	/// <https://schema.org/Chiropractic>
	Chiropractic,
	/// <https://schema.org/Homeopathic>
	Homeopathic,
	/// <https://schema.org/Osteopathic>
	Osteopathic,
	/// <https://schema.org/TraditionalChinese>
	TraditionalChinese,
	/// <https://schema.org/WesternConventional>
	WesternConventional,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicineSystem {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicineSystem::Ayurvedic => {
					serializer.serialize_unit_variant("MedicineSystem", 0u32, "Ayurvedic")
				}
				MedicineSystem::Chiropractic => {
					serializer.serialize_unit_variant("MedicineSystem", 1u32, "Chiropractic")
				}
				MedicineSystem::Homeopathic => {
					serializer.serialize_unit_variant("MedicineSystem", 2u32, "Homeopathic")
				}
				MedicineSystem::Osteopathic => {
					serializer.serialize_unit_variant("MedicineSystem", 3u32, "Osteopathic")
				}
				MedicineSystem::TraditionalChinese => {
					serializer.serialize_unit_variant("MedicineSystem", 4u32, "TraditionalChinese")
				}
				MedicineSystem::WesternConventional => {
					serializer.serialize_unit_variant("MedicineSystem", 5u32, "WesternConventional")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicineSystem {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Ayurvedic,
				Chiropractic,
				Homeopathic,
				Osteopathic,
				TraditionalChinese,
				WesternConventional,
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
						"Ayurvedic" => Ok(Field::Ayurvedic),
						"Chiropractic" => Ok(Field::Chiropractic),
						"Homeopathic" => Ok(Field::Homeopathic),
						"Osteopathic" => Ok(Field::Osteopathic),
						"TraditionalChinese" => Ok(Field::TraditionalChinese),
						"WesternConventional" => Ok(Field::WesternConventional),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Ayurvedic" => Ok(Field::Ayurvedic),
						b"Chiropractic" => Ok(Field::Chiropractic),
						b"Homeopathic" => Ok(Field::Homeopathic),
						b"Osteopathic" => Ok(Field::Osteopathic),
						b"TraditionalChinese" => Ok(Field::TraditionalChinese),
						b"WesternConventional" => Ok(Field::WesternConventional),
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
				type Value = MedicineSystem;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MedicineSystem")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Ayurvedic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicineSystem::Ayurvedic)
						}
						(Field::Chiropractic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicineSystem::Chiropractic)
						}
						(Field::Homeopathic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicineSystem::Homeopathic)
						}
						(Field::Osteopathic, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicineSystem::Osteopathic)
						}
						(Field::TraditionalChinese, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicineSystem::TraditionalChinese)
						}
						(Field::WesternConventional, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MedicineSystem::WesternConventional)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"Ayurvedic",
				"Chiropractic",
				"Homeopathic",
				"Osteopathic",
				"TraditionalChinese",
				"WesternConventional",
			];
			deserializer.deserialize_enum("MedicineSystem", VARIANTS, EnumerationVisitor)
		}
	}
}

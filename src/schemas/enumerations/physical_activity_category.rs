/// <https://schema.org/PhysicalActivityCategory>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PhysicalActivityCategory {
	/// <https://schema.org/AerobicActivity>
	AerobicActivity,
	/// <https://schema.org/AnaerobicActivity>
	AnaerobicActivity,
	/// <https://schema.org/Balance>
	Balance,
	/// <https://schema.org/Flexibility>
	Flexibility,
	/// <https://schema.org/LeisureTimeActivity>
	LeisureTimeActivity,
	/// <https://schema.org/OccupationalActivity>
	OccupationalActivity,
	/// <https://schema.org/StrengthTraining>
	StrengthTraining,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for PhysicalActivityCategory {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PhysicalActivityCategory::AerobicActivity => serializer.serialize_unit_variant(
					"PhysicalActivityCategory",
					0u32,
					"AerobicActivity",
				),
				PhysicalActivityCategory::AnaerobicActivity => serializer.serialize_unit_variant(
					"PhysicalActivityCategory",
					1u32,
					"AnaerobicActivity",
				),
				PhysicalActivityCategory::Balance => {
					serializer.serialize_unit_variant("PhysicalActivityCategory", 2u32, "Balance")
				}
				PhysicalActivityCategory::Flexibility => serializer.serialize_unit_variant(
					"PhysicalActivityCategory",
					3u32,
					"Flexibility",
				),
				PhysicalActivityCategory::LeisureTimeActivity => serializer.serialize_unit_variant(
					"PhysicalActivityCategory",
					4u32,
					"LeisureTimeActivity",
				),
				PhysicalActivityCategory::OccupationalActivity => serializer
					.serialize_unit_variant(
						"PhysicalActivityCategory",
						5u32,
						"OccupationalActivity",
					),
				PhysicalActivityCategory::StrengthTraining => serializer.serialize_unit_variant(
					"PhysicalActivityCategory",
					6u32,
					"StrengthTraining",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for PhysicalActivityCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AerobicActivity,
				AnaerobicActivity,
				Balance,
				Flexibility,
				LeisureTimeActivity,
				OccupationalActivity,
				StrengthTraining,
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
						"AerobicActivity" => Ok(Field::AerobicActivity),
						"AnaerobicActivity" => Ok(Field::AnaerobicActivity),
						"Balance" => Ok(Field::Balance),
						"Flexibility" => Ok(Field::Flexibility),
						"LeisureTimeActivity" => Ok(Field::LeisureTimeActivity),
						"OccupationalActivity" => Ok(Field::OccupationalActivity),
						"StrengthTraining" => Ok(Field::StrengthTraining),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AerobicActivity" => Ok(Field::AerobicActivity),
						b"AnaerobicActivity" => Ok(Field::AnaerobicActivity),
						b"Balance" => Ok(Field::Balance),
						b"Flexibility" => Ok(Field::Flexibility),
						b"LeisureTimeActivity" => Ok(Field::LeisureTimeActivity),
						b"OccupationalActivity" => Ok(Field::OccupationalActivity),
						b"StrengthTraining" => Ok(Field::StrengthTraining),
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
				type Value = PhysicalActivityCategory;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema PhysicalActivityCategory")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AerobicActivity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalActivityCategory::AerobicActivity)
						}
						(Field::AnaerobicActivity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalActivityCategory::AnaerobicActivity)
						}
						(Field::Balance, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalActivityCategory::Balance)
						}
						(Field::Flexibility, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalActivityCategory::Flexibility)
						}
						(Field::LeisureTimeActivity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalActivityCategory::LeisureTimeActivity)
						}
						(Field::OccupationalActivity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalActivityCategory::OccupationalActivity)
						}
						(Field::StrengthTraining, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PhysicalActivityCategory::StrengthTraining)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AerobicActivity",
				"AnaerobicActivity",
				"Balance",
				"Flexibility",
				"LeisureTimeActivity",
				"OccupationalActivity",
				"StrengthTraining",
			];
			deserializer.deserialize_enum("PhysicalActivityCategory", VARIANTS, EnumerationVisitor)
		}
	}
}

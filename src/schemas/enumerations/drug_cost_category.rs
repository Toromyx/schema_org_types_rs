/// <https://schema.org/DrugCostCategory>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DrugCostCategory {
	/// <https://schema.org/ReimbursementCap>
	ReimbursementCap,
	/// <https://schema.org/Retail>
	Retail,
	/// <https://schema.org/Wholesale>
	Wholesale,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DrugCostCategory {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DrugCostCategory::ReimbursementCap => {
					serializer.serialize_unit_variant("DrugCostCategory", 0u32, "ReimbursementCap")
				}
				DrugCostCategory::Retail => {
					serializer.serialize_unit_variant("DrugCostCategory", 1u32, "Retail")
				}
				DrugCostCategory::Wholesale => {
					serializer.serialize_unit_variant("DrugCostCategory", 2u32, "Wholesale")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for DrugCostCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ReimbursementCap,
				Retail,
				Wholesale,
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
						"ReimbursementCap" => Ok(Field::ReimbursementCap),
						"Retail" => Ok(Field::Retail),
						"Wholesale" => Ok(Field::Wholesale),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ReimbursementCap" => Ok(Field::ReimbursementCap),
						b"Retail" => Ok(Field::Retail),
						b"Wholesale" => Ok(Field::Wholesale),
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
				type Value = DrugCostCategory;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DrugCostCategory")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ReimbursementCap, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugCostCategory::ReimbursementCap)
						}
						(Field::Retail, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugCostCategory::Retail)
						}
						(Field::Wholesale, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugCostCategory::Wholesale)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["ReimbursementCap", "Retail", "Wholesale"];
			deserializer.deserialize_enum("DrugCostCategory", VARIANTS, EnumerationVisitor)
		}
	}
}

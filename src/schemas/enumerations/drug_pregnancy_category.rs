/// <https://schema.org/DrugPregnancyCategory>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DrugPregnancyCategory {
	/// <https://schema.org/FDAcategoryA>
	FdAcategoryA,
	/// <https://schema.org/FDAcategoryB>
	FdAcategoryB,
	/// <https://schema.org/FDAcategoryC>
	FdAcategoryC,
	/// <https://schema.org/FDAcategoryD>
	FdAcategoryD,
	/// <https://schema.org/FDAcategoryX>
	FdAcategoryX,
	/// <https://schema.org/FDAnotEvaluated>
	FdAnotEvaluated,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DrugPregnancyCategory {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DrugPregnancyCategory::FdAcategoryA => {
					serializer.serialize_unit_variant("DrugPregnancyCategory", 0u32, "FdAcategoryA")
				}
				DrugPregnancyCategory::FdAcategoryB => {
					serializer.serialize_unit_variant("DrugPregnancyCategory", 1u32, "FdAcategoryB")
				}
				DrugPregnancyCategory::FdAcategoryC => {
					serializer.serialize_unit_variant("DrugPregnancyCategory", 2u32, "FdAcategoryC")
				}
				DrugPregnancyCategory::FdAcategoryD => {
					serializer.serialize_unit_variant("DrugPregnancyCategory", 3u32, "FdAcategoryD")
				}
				DrugPregnancyCategory::FdAcategoryX => {
					serializer.serialize_unit_variant("DrugPregnancyCategory", 4u32, "FdAcategoryX")
				}
				DrugPregnancyCategory::FdAnotEvaluated => serializer.serialize_unit_variant(
					"DrugPregnancyCategory",
					5u32,
					"FdAnotEvaluated",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for DrugPregnancyCategory {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				FdAcategoryA,
				FdAcategoryB,
				FdAcategoryC,
				FdAcategoryD,
				FdAcategoryX,
				FdAnotEvaluated,
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
						"FdAcategoryA" => Ok(Field::FdAcategoryA),
						"FdAcategoryB" => Ok(Field::FdAcategoryB),
						"FdAcategoryC" => Ok(Field::FdAcategoryC),
						"FdAcategoryD" => Ok(Field::FdAcategoryD),
						"FdAcategoryX" => Ok(Field::FdAcategoryX),
						"FdAnotEvaluated" => Ok(Field::FdAnotEvaluated),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"FdAcategoryA" => Ok(Field::FdAcategoryA),
						b"FdAcategoryB" => Ok(Field::FdAcategoryB),
						b"FdAcategoryC" => Ok(Field::FdAcategoryC),
						b"FdAcategoryD" => Ok(Field::FdAcategoryD),
						b"FdAcategoryX" => Ok(Field::FdAcategoryX),
						b"FdAnotEvaluated" => Ok(Field::FdAnotEvaluated),
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
				type Value = DrugPregnancyCategory;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DrugPregnancyCategory")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::FdAcategoryA, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugPregnancyCategory::FdAcategoryA)
						}
						(Field::FdAcategoryB, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugPregnancyCategory::FdAcategoryB)
						}
						(Field::FdAcategoryC, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugPregnancyCategory::FdAcategoryC)
						}
						(Field::FdAcategoryD, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugPregnancyCategory::FdAcategoryD)
						}
						(Field::FdAcategoryX, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugPregnancyCategory::FdAcategoryX)
						}
						(Field::FdAnotEvaluated, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DrugPregnancyCategory::FdAnotEvaluated)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"FdAcategoryA",
				"FdAcategoryB",
				"FdAcategoryC",
				"FdAcategoryD",
				"FdAcategoryX",
				"FdAnotEvaluated",
			];
			deserializer.deserialize_enum("DrugPregnancyCategory", VARIANTS, EnumerationVisitor)
		}
	}
}

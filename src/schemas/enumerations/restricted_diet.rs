/// <https://schema.org/RestrictedDiet>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum RestrictedDiet {
	/// <https://schema.org/DiabeticDiet>
	DiabeticDiet,
	/// <https://schema.org/GlutenFreeDiet>
	GlutenFreeDiet,
	/// <https://schema.org/HalalDiet>
	HalalDiet,
	/// <https://schema.org/HinduDiet>
	HinduDiet,
	/// <https://schema.org/KosherDiet>
	KosherDiet,
	/// <https://schema.org/LowCalorieDiet>
	LowCalorieDiet,
	/// <https://schema.org/LowFatDiet>
	LowFatDiet,
	/// <https://schema.org/LowLactoseDiet>
	LowLactoseDiet,
	/// <https://schema.org/LowSaltDiet>
	LowSaltDiet,
	/// <https://schema.org/VeganDiet>
	VeganDiet,
	/// <https://schema.org/VegetarianDiet>
	VegetarianDiet,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for RestrictedDiet {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				RestrictedDiet::DiabeticDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 0u32, "DiabeticDiet")
				}
				RestrictedDiet::GlutenFreeDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 1u32, "GlutenFreeDiet")
				}
				RestrictedDiet::HalalDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 2u32, "HalalDiet")
				}
				RestrictedDiet::HinduDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 3u32, "HinduDiet")
				}
				RestrictedDiet::KosherDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 4u32, "KosherDiet")
				}
				RestrictedDiet::LowCalorieDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 5u32, "LowCalorieDiet")
				}
				RestrictedDiet::LowFatDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 6u32, "LowFatDiet")
				}
				RestrictedDiet::LowLactoseDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 7u32, "LowLactoseDiet")
				}
				RestrictedDiet::LowSaltDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 8u32, "LowSaltDiet")
				}
				RestrictedDiet::VeganDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 9u32, "VeganDiet")
				}
				RestrictedDiet::VegetarianDiet => {
					serializer.serialize_unit_variant("RestrictedDiet", 10u32, "VegetarianDiet")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for RestrictedDiet {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				DiabeticDiet,
				GlutenFreeDiet,
				HalalDiet,
				HinduDiet,
				KosherDiet,
				LowCalorieDiet,
				LowFatDiet,
				LowLactoseDiet,
				LowSaltDiet,
				VeganDiet,
				VegetarianDiet,
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
						"DiabeticDiet" => Ok(Field::DiabeticDiet),
						"GlutenFreeDiet" => Ok(Field::GlutenFreeDiet),
						"HalalDiet" => Ok(Field::HalalDiet),
						"HinduDiet" => Ok(Field::HinduDiet),
						"KosherDiet" => Ok(Field::KosherDiet),
						"LowCalorieDiet" => Ok(Field::LowCalorieDiet),
						"LowFatDiet" => Ok(Field::LowFatDiet),
						"LowLactoseDiet" => Ok(Field::LowLactoseDiet),
						"LowSaltDiet" => Ok(Field::LowSaltDiet),
						"VeganDiet" => Ok(Field::VeganDiet),
						"VegetarianDiet" => Ok(Field::VegetarianDiet),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"DiabeticDiet" => Ok(Field::DiabeticDiet),
						b"GlutenFreeDiet" => Ok(Field::GlutenFreeDiet),
						b"HalalDiet" => Ok(Field::HalalDiet),
						b"HinduDiet" => Ok(Field::HinduDiet),
						b"KosherDiet" => Ok(Field::KosherDiet),
						b"LowCalorieDiet" => Ok(Field::LowCalorieDiet),
						b"LowFatDiet" => Ok(Field::LowFatDiet),
						b"LowLactoseDiet" => Ok(Field::LowLactoseDiet),
						b"LowSaltDiet" => Ok(Field::LowSaltDiet),
						b"VeganDiet" => Ok(Field::VeganDiet),
						b"VegetarianDiet" => Ok(Field::VegetarianDiet),
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
				type Value = RestrictedDiet;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema RestrictedDiet")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::DiabeticDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::DiabeticDiet)
						}
						(Field::GlutenFreeDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::GlutenFreeDiet)
						}
						(Field::HalalDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::HalalDiet)
						}
						(Field::HinduDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::HinduDiet)
						}
						(Field::KosherDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::KosherDiet)
						}
						(Field::LowCalorieDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::LowCalorieDiet)
						}
						(Field::LowFatDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::LowFatDiet)
						}
						(Field::LowLactoseDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::LowLactoseDiet)
						}
						(Field::LowSaltDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::LowSaltDiet)
						}
						(Field::VeganDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::VeganDiet)
						}
						(Field::VegetarianDiet, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(RestrictedDiet::VegetarianDiet)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"DiabeticDiet",
				"GlutenFreeDiet",
				"HalalDiet",
				"HinduDiet",
				"KosherDiet",
				"LowCalorieDiet",
				"LowFatDiet",
				"LowLactoseDiet",
				"LowSaltDiet",
				"VeganDiet",
				"VegetarianDiet",
			];
			deserializer.deserialize_enum("RestrictedDiet", VARIANTS, EnumerationVisitor)
		}
	}
}

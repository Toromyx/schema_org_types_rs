/// <https://schema.org/InfectiousAgentClass>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum InfectiousAgentClass {
	/// <https://schema.org/Bacteria>
	Bacteria,
	/// <https://schema.org/Fungus>
	Fungus,
	/// <https://schema.org/MulticellularParasite>
	MulticellularParasite,
	/// <https://schema.org/Prion>
	Prion,
	/// <https://schema.org/Protozoa>
	Protozoa,
	/// <https://schema.org/Virus>
	Virus,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for InfectiousAgentClass {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				InfectiousAgentClass::Bacteria => {
					serializer.serialize_unit_variant("InfectiousAgentClass", 0u32, "Bacteria")
				}
				InfectiousAgentClass::Fungus => {
					serializer.serialize_unit_variant("InfectiousAgentClass", 1u32, "Fungus")
				}
				InfectiousAgentClass::MulticellularParasite => serializer.serialize_unit_variant(
					"InfectiousAgentClass",
					2u32,
					"MulticellularParasite",
				),
				InfectiousAgentClass::Prion => {
					serializer.serialize_unit_variant("InfectiousAgentClass", 3u32, "Prion")
				}
				InfectiousAgentClass::Protozoa => {
					serializer.serialize_unit_variant("InfectiousAgentClass", 4u32, "Protozoa")
				}
				InfectiousAgentClass::Virus => {
					serializer.serialize_unit_variant("InfectiousAgentClass", 5u32, "Virus")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for InfectiousAgentClass {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Bacteria,
				Fungus,
				MulticellularParasite,
				Prion,
				Protozoa,
				Virus,
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
						"Bacteria" => Ok(Field::Bacteria),
						"Fungus" => Ok(Field::Fungus),
						"MulticellularParasite" => Ok(Field::MulticellularParasite),
						"Prion" => Ok(Field::Prion),
						"Protozoa" => Ok(Field::Protozoa),
						"Virus" => Ok(Field::Virus),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Bacteria" => Ok(Field::Bacteria),
						b"Fungus" => Ok(Field::Fungus),
						b"MulticellularParasite" => Ok(Field::MulticellularParasite),
						b"Prion" => Ok(Field::Prion),
						b"Protozoa" => Ok(Field::Protozoa),
						b"Virus" => Ok(Field::Virus),
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
				type Value = InfectiousAgentClass;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema InfectiousAgentClass")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Bacteria, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(InfectiousAgentClass::Bacteria)
						}
						(Field::Fungus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(InfectiousAgentClass::Fungus)
						}
						(Field::MulticellularParasite, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(InfectiousAgentClass::MulticellularParasite)
						}
						(Field::Prion, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(InfectiousAgentClass::Prion)
						}
						(Field::Protozoa, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(InfectiousAgentClass::Protozoa)
						}
						(Field::Virus, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(InfectiousAgentClass::Virus)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"Bacteria",
				"Fungus",
				"MulticellularParasite",
				"Prion",
				"Protozoa",
				"Virus",
			];
			deserializer.deserialize_enum("InfectiousAgentClass", VARIANTS, EnumerationVisitor)
		}
	}
}

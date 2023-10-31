/// <https://schema.org/EnergyStarEnergyEfficiencyEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum EnergyStarEnergyEfficiencyEnumeration {
	/// <https://schema.org/EnergyStarCertified>
	EnergyStarCertified,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for EnergyStarEnergyEfficiencyEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				EnergyStarEnergyEfficiencyEnumeration::EnergyStarCertified => serializer
					.serialize_unit_variant(
						"EnergyStarEnergyEfficiencyEnumeration",
						0u32,
						"EnergyStarCertified",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for EnergyStarEnergyEfficiencyEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				EnergyStarCertified,
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
						"EnergyStarCertified" => Ok(Field::EnergyStarCertified),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"EnergyStarCertified" => Ok(Field::EnergyStarCertified),
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
				type Value = EnergyStarEnergyEfficiencyEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema EnergyStarEnergyEfficiencyEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::EnergyStarCertified, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EnergyStarEnergyEfficiencyEnumeration::EnergyStarCertified)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["EnergyStarCertified"];
			deserializer.deserialize_enum(
				"EnergyStarEnergyEfficiencyEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

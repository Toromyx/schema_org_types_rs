/// <https://schema.org/DriveWheelConfigurationValue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DriveWheelConfigurationValue {
	/// <https://schema.org/AllWheelDriveConfiguration>
	AllWheelDriveConfiguration,
	/// <https://schema.org/FourWheelDriveConfiguration>
	FourWheelDriveConfiguration,
	/// <https://schema.org/FrontWheelDriveConfiguration>
	FrontWheelDriveConfiguration,
	/// <https://schema.org/RearWheelDriveConfiguration>
	RearWheelDriveConfiguration,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DriveWheelConfigurationValue {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DriveWheelConfigurationValue::AllWheelDriveConfiguration => serializer
					.serialize_unit_variant(
						"DriveWheelConfigurationValue",
						0u32,
						"AllWheelDriveConfiguration",
					),
				DriveWheelConfigurationValue::FourWheelDriveConfiguration => serializer
					.serialize_unit_variant(
						"DriveWheelConfigurationValue",
						1u32,
						"FourWheelDriveConfiguration",
					),
				DriveWheelConfigurationValue::FrontWheelDriveConfiguration => serializer
					.serialize_unit_variant(
						"DriveWheelConfigurationValue",
						2u32,
						"FrontWheelDriveConfiguration",
					),
				DriveWheelConfigurationValue::RearWheelDriveConfiguration => serializer
					.serialize_unit_variant(
						"DriveWheelConfigurationValue",
						3u32,
						"RearWheelDriveConfiguration",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for DriveWheelConfigurationValue {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AllWheelDriveConfiguration,
				FourWheelDriveConfiguration,
				FrontWheelDriveConfiguration,
				RearWheelDriveConfiguration,
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
						"AllWheelDriveConfiguration" => Ok(Field::AllWheelDriveConfiguration),
						"FourWheelDriveConfiguration" => Ok(Field::FourWheelDriveConfiguration),
						"FrontWheelDriveConfiguration" => Ok(Field::FrontWheelDriveConfiguration),
						"RearWheelDriveConfiguration" => Ok(Field::RearWheelDriveConfiguration),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AllWheelDriveConfiguration" => Ok(Field::AllWheelDriveConfiguration),
						b"FourWheelDriveConfiguration" => Ok(Field::FourWheelDriveConfiguration),
						b"FrontWheelDriveConfiguration" => Ok(Field::FrontWheelDriveConfiguration),
						b"RearWheelDriveConfiguration" => Ok(Field::RearWheelDriveConfiguration),
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
				type Value = DriveWheelConfigurationValue;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DriveWheelConfigurationValue")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AllWheelDriveConfiguration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DriveWheelConfigurationValue::AllWheelDriveConfiguration)
						}
						(Field::FourWheelDriveConfiguration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DriveWheelConfigurationValue::FourWheelDriveConfiguration)
						}
						(Field::FrontWheelDriveConfiguration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DriveWheelConfigurationValue::FrontWheelDriveConfiguration)
						}
						(Field::RearWheelDriveConfiguration, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DriveWheelConfigurationValue::RearWheelDriveConfiguration)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AllWheelDriveConfiguration",
				"FourWheelDriveConfiguration",
				"FrontWheelDriveConfiguration",
				"RearWheelDriveConfiguration",
			];
			deserializer.deserialize_enum(
				"DriveWheelConfigurationValue",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

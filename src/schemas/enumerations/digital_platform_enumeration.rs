/// <https://schema.org/DigitalPlatformEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DigitalPlatformEnumeration {
	/// <https://schema.org/AndroidPlatform>
	AndroidPlatform,
	/// <https://schema.org/DesktopWebPlatform>
	DesktopWebPlatform,
	/// <https://schema.org/GenericWebPlatform>
	GenericWebPlatform,
	/// <https://schema.org/IOSPlatform>
	IosPlatform,
	/// <https://schema.org/MobileWebPlatform>
	MobileWebPlatform,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DigitalPlatformEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DigitalPlatformEnumeration::AndroidPlatform => serializer.serialize_unit_variant(
					"DigitalPlatformEnumeration",
					0u32,
					"AndroidPlatform",
				),
				DigitalPlatformEnumeration::DesktopWebPlatform => serializer
					.serialize_unit_variant(
						"DigitalPlatformEnumeration",
						1u32,
						"DesktopWebPlatform",
					),
				DigitalPlatformEnumeration::GenericWebPlatform => serializer
					.serialize_unit_variant(
						"DigitalPlatformEnumeration",
						2u32,
						"GenericWebPlatform",
					),
				DigitalPlatformEnumeration::IosPlatform => serializer.serialize_unit_variant(
					"DigitalPlatformEnumeration",
					3u32,
					"IosPlatform",
				),
				DigitalPlatformEnumeration::MobileWebPlatform => serializer.serialize_unit_variant(
					"DigitalPlatformEnumeration",
					4u32,
					"MobileWebPlatform",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for DigitalPlatformEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AndroidPlatform,
				DesktopWebPlatform,
				GenericWebPlatform,
				IosPlatform,
				MobileWebPlatform,
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
						"AndroidPlatform" => Ok(Field::AndroidPlatform),
						"DesktopWebPlatform" => Ok(Field::DesktopWebPlatform),
						"GenericWebPlatform" => Ok(Field::GenericWebPlatform),
						"IosPlatform" => Ok(Field::IosPlatform),
						"MobileWebPlatform" => Ok(Field::MobileWebPlatform),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AndroidPlatform" => Ok(Field::AndroidPlatform),
						b"DesktopWebPlatform" => Ok(Field::DesktopWebPlatform),
						b"GenericWebPlatform" => Ok(Field::GenericWebPlatform),
						b"IosPlatform" => Ok(Field::IosPlatform),
						b"MobileWebPlatform" => Ok(Field::MobileWebPlatform),
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
				type Value = DigitalPlatformEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DigitalPlatformEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AndroidPlatform, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DigitalPlatformEnumeration::AndroidPlatform)
						}
						(Field::DesktopWebPlatform, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DigitalPlatformEnumeration::DesktopWebPlatform)
						}
						(Field::GenericWebPlatform, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DigitalPlatformEnumeration::GenericWebPlatform)
						}
						(Field::IosPlatform, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DigitalPlatformEnumeration::IosPlatform)
						}
						(Field::MobileWebPlatform, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DigitalPlatformEnumeration::MobileWebPlatform)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AndroidPlatform",
				"DesktopWebPlatform",
				"GenericWebPlatform",
				"IosPlatform",
				"MobileWebPlatform",
			];
			deserializer.deserialize_enum(
				"DigitalPlatformEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

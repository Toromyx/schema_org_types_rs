/// <https://schema.org/MusicReleaseFormatType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MusicReleaseFormatType {
	/// <https://schema.org/CDFormat>
	CdFormat,
	/// <https://schema.org/CassetteFormat>
	CassetteFormat,
	/// <https://schema.org/DVDFormat>
	DvdFormat,
	/// <https://schema.org/DigitalAudioTapeFormat>
	DigitalAudioTapeFormat,
	/// <https://schema.org/DigitalFormat>
	DigitalFormat,
	/// <https://schema.org/LaserDiscFormat>
	LaserDiscFormat,
	/// <https://schema.org/VinylFormat>
	VinylFormat,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MusicReleaseFormatType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MusicReleaseFormatType::CdFormat => {
					serializer.serialize_unit_variant("MusicReleaseFormatType", 0u32, "CdFormat")
				}
				MusicReleaseFormatType::CassetteFormat => serializer.serialize_unit_variant(
					"MusicReleaseFormatType",
					1u32,
					"CassetteFormat",
				),
				MusicReleaseFormatType::DvdFormat => {
					serializer.serialize_unit_variant("MusicReleaseFormatType", 2u32, "DvdFormat")
				}
				MusicReleaseFormatType::DigitalAudioTapeFormat => serializer
					.serialize_unit_variant(
						"MusicReleaseFormatType",
						3u32,
						"DigitalAudioTapeFormat",
					),
				MusicReleaseFormatType::DigitalFormat => serializer.serialize_unit_variant(
					"MusicReleaseFormatType",
					4u32,
					"DigitalFormat",
				),
				MusicReleaseFormatType::LaserDiscFormat => serializer.serialize_unit_variant(
					"MusicReleaseFormatType",
					5u32,
					"LaserDiscFormat",
				),
				MusicReleaseFormatType::VinylFormat => {
					serializer.serialize_unit_variant("MusicReleaseFormatType", 6u32, "VinylFormat")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for MusicReleaseFormatType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				CdFormat,
				CassetteFormat,
				DvdFormat,
				DigitalAudioTapeFormat,
				DigitalFormat,
				LaserDiscFormat,
				VinylFormat,
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
						"CdFormat" => Ok(Field::CdFormat),
						"CassetteFormat" => Ok(Field::CassetteFormat),
						"DvdFormat" => Ok(Field::DvdFormat),
						"DigitalAudioTapeFormat" => Ok(Field::DigitalAudioTapeFormat),
						"DigitalFormat" => Ok(Field::DigitalFormat),
						"LaserDiscFormat" => Ok(Field::LaserDiscFormat),
						"VinylFormat" => Ok(Field::VinylFormat),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"CdFormat" => Ok(Field::CdFormat),
						b"CassetteFormat" => Ok(Field::CassetteFormat),
						b"DvdFormat" => Ok(Field::DvdFormat),
						b"DigitalAudioTapeFormat" => Ok(Field::DigitalAudioTapeFormat),
						b"DigitalFormat" => Ok(Field::DigitalFormat),
						b"LaserDiscFormat" => Ok(Field::LaserDiscFormat),
						b"VinylFormat" => Ok(Field::VinylFormat),
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
				type Value = MusicReleaseFormatType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MusicReleaseFormatType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::CdFormat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicReleaseFormatType::CdFormat)
						}
						(Field::CassetteFormat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicReleaseFormatType::CassetteFormat)
						}
						(Field::DvdFormat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicReleaseFormatType::DvdFormat)
						}
						(Field::DigitalAudioTapeFormat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicReleaseFormatType::DigitalAudioTapeFormat)
						}
						(Field::DigitalFormat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicReleaseFormatType::DigitalFormat)
						}
						(Field::LaserDiscFormat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicReleaseFormatType::LaserDiscFormat)
						}
						(Field::VinylFormat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicReleaseFormatType::VinylFormat)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"CdFormat",
				"CassetteFormat",
				"DvdFormat",
				"DigitalAudioTapeFormat",
				"DigitalFormat",
				"LaserDiscFormat",
				"VinylFormat",
			];
			deserializer.deserialize_enum("MusicReleaseFormatType", VARIANTS, EnumerationVisitor)
		}
	}
}

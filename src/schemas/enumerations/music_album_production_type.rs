/// <https://schema.org/MusicAlbumProductionType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MusicAlbumProductionType {
	/// <https://schema.org/CompilationAlbum>
	CompilationAlbum,
	/// <https://schema.org/DJMixAlbum>
	DjMixAlbum,
	/// <https://schema.org/DemoAlbum>
	DemoAlbum,
	/// <https://schema.org/LiveAlbum>
	LiveAlbum,
	/// <https://schema.org/MixtapeAlbum>
	MixtapeAlbum,
	/// <https://schema.org/RemixAlbum>
	RemixAlbum,
	/// <https://schema.org/SoundtrackAlbum>
	SoundtrackAlbum,
	/// <https://schema.org/SpokenWordAlbum>
	SpokenWordAlbum,
	/// <https://schema.org/StudioAlbum>
	StudioAlbum,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MusicAlbumProductionType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MusicAlbumProductionType::CompilationAlbum => serializer.serialize_unit_variant(
					"MusicAlbumProductionType",
					0u32,
					"CompilationAlbum",
				),
				MusicAlbumProductionType::DjMixAlbum => serializer.serialize_unit_variant(
					"MusicAlbumProductionType",
					1u32,
					"DjMixAlbum",
				),
				MusicAlbumProductionType::DemoAlbum => {
					serializer.serialize_unit_variant("MusicAlbumProductionType", 2u32, "DemoAlbum")
				}
				MusicAlbumProductionType::LiveAlbum => {
					serializer.serialize_unit_variant("MusicAlbumProductionType", 3u32, "LiveAlbum")
				}
				MusicAlbumProductionType::MixtapeAlbum => serializer.serialize_unit_variant(
					"MusicAlbumProductionType",
					4u32,
					"MixtapeAlbum",
				),
				MusicAlbumProductionType::RemixAlbum => serializer.serialize_unit_variant(
					"MusicAlbumProductionType",
					5u32,
					"RemixAlbum",
				),
				MusicAlbumProductionType::SoundtrackAlbum => serializer.serialize_unit_variant(
					"MusicAlbumProductionType",
					6u32,
					"SoundtrackAlbum",
				),
				MusicAlbumProductionType::SpokenWordAlbum => serializer.serialize_unit_variant(
					"MusicAlbumProductionType",
					7u32,
					"SpokenWordAlbum",
				),
				MusicAlbumProductionType::StudioAlbum => serializer.serialize_unit_variant(
					"MusicAlbumProductionType",
					8u32,
					"StudioAlbum",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for MusicAlbumProductionType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				CompilationAlbum,
				DjMixAlbum,
				DemoAlbum,
				LiveAlbum,
				MixtapeAlbum,
				RemixAlbum,
				SoundtrackAlbum,
				SpokenWordAlbum,
				StudioAlbum,
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
						"CompilationAlbum" => Ok(Field::CompilationAlbum),
						"DjMixAlbum" => Ok(Field::DjMixAlbum),
						"DemoAlbum" => Ok(Field::DemoAlbum),
						"LiveAlbum" => Ok(Field::LiveAlbum),
						"MixtapeAlbum" => Ok(Field::MixtapeAlbum),
						"RemixAlbum" => Ok(Field::RemixAlbum),
						"SoundtrackAlbum" => Ok(Field::SoundtrackAlbum),
						"SpokenWordAlbum" => Ok(Field::SpokenWordAlbum),
						"StudioAlbum" => Ok(Field::StudioAlbum),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"CompilationAlbum" => Ok(Field::CompilationAlbum),
						b"DjMixAlbum" => Ok(Field::DjMixAlbum),
						b"DemoAlbum" => Ok(Field::DemoAlbum),
						b"LiveAlbum" => Ok(Field::LiveAlbum),
						b"MixtapeAlbum" => Ok(Field::MixtapeAlbum),
						b"RemixAlbum" => Ok(Field::RemixAlbum),
						b"SoundtrackAlbum" => Ok(Field::SoundtrackAlbum),
						b"SpokenWordAlbum" => Ok(Field::SpokenWordAlbum),
						b"StudioAlbum" => Ok(Field::StudioAlbum),
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
				type Value = MusicAlbumProductionType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MusicAlbumProductionType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::CompilationAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::CompilationAlbum)
						}
						(Field::DjMixAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::DjMixAlbum)
						}
						(Field::DemoAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::DemoAlbum)
						}
						(Field::LiveAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::LiveAlbum)
						}
						(Field::MixtapeAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::MixtapeAlbum)
						}
						(Field::RemixAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::RemixAlbum)
						}
						(Field::SoundtrackAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::SoundtrackAlbum)
						}
						(Field::SpokenWordAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::SpokenWordAlbum)
						}
						(Field::StudioAlbum, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumProductionType::StudioAlbum)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"CompilationAlbum",
				"DjMixAlbum",
				"DemoAlbum",
				"LiveAlbum",
				"MixtapeAlbum",
				"RemixAlbum",
				"SoundtrackAlbum",
				"SpokenWordAlbum",
				"StudioAlbum",
			];
			deserializer.deserialize_enum("MusicAlbumProductionType", VARIANTS, EnumerationVisitor)
		}
	}
}

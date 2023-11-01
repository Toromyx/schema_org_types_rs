/// <https://schema.org/MusicAlbumReleaseType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MusicAlbumReleaseType {
	/// <https://schema.org/AlbumRelease>
	AlbumRelease,
	/// <https://schema.org/BroadcastRelease>
	BroadcastRelease,
	/// <https://schema.org/EPRelease>
	EpRelease,
	/// <https://schema.org/SingleRelease>
	SingleRelease,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MusicAlbumReleaseType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MusicAlbumReleaseType::AlbumRelease => {
					serializer.serialize_unit_variant("MusicAlbumReleaseType", 0u32, "AlbumRelease")
				}
				MusicAlbumReleaseType::BroadcastRelease => serializer.serialize_unit_variant(
					"MusicAlbumReleaseType",
					1u32,
					"BroadcastRelease",
				),
				MusicAlbumReleaseType::EpRelease => {
					serializer.serialize_unit_variant("MusicAlbumReleaseType", 2u32, "EpRelease")
				}
				MusicAlbumReleaseType::SingleRelease => serializer.serialize_unit_variant(
					"MusicAlbumReleaseType",
					3u32,
					"SingleRelease",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for MusicAlbumReleaseType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AlbumRelease,
				BroadcastRelease,
				EpRelease,
				SingleRelease,
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
						"AlbumRelease" => Ok(Field::AlbumRelease),
						"BroadcastRelease" => Ok(Field::BroadcastRelease),
						"EpRelease" => Ok(Field::EpRelease),
						"SingleRelease" => Ok(Field::SingleRelease),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AlbumRelease" => Ok(Field::AlbumRelease),
						b"BroadcastRelease" => Ok(Field::BroadcastRelease),
						b"EpRelease" => Ok(Field::EpRelease),
						b"SingleRelease" => Ok(Field::SingleRelease),
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
				type Value = MusicAlbumReleaseType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MusicAlbumReleaseType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AlbumRelease, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumReleaseType::AlbumRelease)
						}
						(Field::BroadcastRelease, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumReleaseType::BroadcastRelease)
						}
						(Field::EpRelease, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumReleaseType::EpRelease)
						}
						(Field::SingleRelease, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MusicAlbumReleaseType::SingleRelease)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AlbumRelease",
				"BroadcastRelease",
				"EpRelease",
				"SingleRelease",
			];
			deserializer.deserialize_enum("MusicAlbumReleaseType", VARIANTS, EnumerationVisitor)
		}
	}
}

/// <https://schema.org/GameServerStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GameServerStatus {
	/// <https://schema.org/OfflinePermanently>
	OfflinePermanently,
	/// <https://schema.org/OfflineTemporarily>
	OfflineTemporarily,
	/// <https://schema.org/Online>
	Online,
	/// <https://schema.org/OnlineFull>
	OnlineFull,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for GameServerStatus {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				GameServerStatus::OfflinePermanently => serializer.serialize_unit_variant(
					"GameServerStatus",
					0u32,
					"OfflinePermanently",
				),
				GameServerStatus::OfflineTemporarily => serializer.serialize_unit_variant(
					"GameServerStatus",
					1u32,
					"OfflineTemporarily",
				),
				GameServerStatus::Online => {
					serializer.serialize_unit_variant("GameServerStatus", 2u32, "Online")
				}
				GameServerStatus::OnlineFull => {
					serializer.serialize_unit_variant("GameServerStatus", 3u32, "OnlineFull")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for GameServerStatus {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				OfflinePermanently,
				OfflineTemporarily,
				Online,
				OnlineFull,
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
						"OfflinePermanently" => Ok(Field::OfflinePermanently),
						"OfflineTemporarily" => Ok(Field::OfflineTemporarily),
						"Online" => Ok(Field::Online),
						"OnlineFull" => Ok(Field::OnlineFull),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"OfflinePermanently" => Ok(Field::OfflinePermanently),
						b"OfflineTemporarily" => Ok(Field::OfflineTemporarily),
						b"Online" => Ok(Field::Online),
						b"OnlineFull" => Ok(Field::OnlineFull),
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
				type Value = GameServerStatus;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema GameServerStatus")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::OfflinePermanently, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GameServerStatus::OfflinePermanently)
						}
						(Field::OfflineTemporarily, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GameServerStatus::OfflineTemporarily)
						}
						(Field::Online, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GameServerStatus::Online)
						}
						(Field::OnlineFull, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GameServerStatus::OnlineFull)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"OfflinePermanently",
				"OfflineTemporarily",
				"Online",
				"OnlineFull",
			];
			deserializer.deserialize_enum("GameServerStatus", VARIANTS, EnumerationVisitor)
		}
	}
}

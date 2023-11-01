/// <https://schema.org/GamePlayMode>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GamePlayMode {
	/// <https://schema.org/CoOp>
	CoOp,
	/// <https://schema.org/MultiPlayer>
	MultiPlayer,
	/// <https://schema.org/SinglePlayer>
	SinglePlayer,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for GamePlayMode {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				GamePlayMode::CoOp => {
					serializer.serialize_unit_variant("GamePlayMode", 0u32, "CoOp")
				}
				GamePlayMode::MultiPlayer => {
					serializer.serialize_unit_variant("GamePlayMode", 1u32, "MultiPlayer")
				}
				GamePlayMode::SinglePlayer => {
					serializer.serialize_unit_variant("GamePlayMode", 2u32, "SinglePlayer")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for GamePlayMode {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				CoOp,
				MultiPlayer,
				SinglePlayer,
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
						"CoOp" => Ok(Field::CoOp),
						"MultiPlayer" => Ok(Field::MultiPlayer),
						"SinglePlayer" => Ok(Field::SinglePlayer),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"CoOp" => Ok(Field::CoOp),
						b"MultiPlayer" => Ok(Field::MultiPlayer),
						b"SinglePlayer" => Ok(Field::SinglePlayer),
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
				type Value = GamePlayMode;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema GamePlayMode")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::CoOp, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GamePlayMode::CoOp)
						}
						(Field::MultiPlayer, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GamePlayMode::MultiPlayer)
						}
						(Field::SinglePlayer, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GamePlayMode::SinglePlayer)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["CoOp", "MultiPlayer", "SinglePlayer"];
			deserializer.deserialize_enum("GamePlayMode", VARIANTS, EnumerationVisitor)
		}
	}
}

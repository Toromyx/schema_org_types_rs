/// <https://schema.org/MediaManipulationRatingEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MediaManipulationRatingEnumeration {
	/// <https://schema.org/DecontextualizedContent>
	DecontextualizedContent,
	/// <https://schema.org/EditedOrCroppedContent>
	EditedOrCroppedContent,
	/// <https://schema.org/OriginalMediaContent>
	OriginalMediaContent,
	/// <https://schema.org/SatireOrParodyContent>
	SatireOrParodyContent,
	/// <https://schema.org/StagedContent>
	StagedContent,
	/// <https://schema.org/TransformedContent>
	TransformedContent,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MediaManipulationRatingEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MediaManipulationRatingEnumeration::DecontextualizedContent => serializer
					.serialize_unit_variant(
						"MediaManipulationRatingEnumeration",
						0u32,
						"DecontextualizedContent",
					),
				MediaManipulationRatingEnumeration::EditedOrCroppedContent => serializer
					.serialize_unit_variant(
						"MediaManipulationRatingEnumeration",
						1u32,
						"EditedOrCroppedContent",
					),
				MediaManipulationRatingEnumeration::OriginalMediaContent => serializer
					.serialize_unit_variant(
						"MediaManipulationRatingEnumeration",
						2u32,
						"OriginalMediaContent",
					),
				MediaManipulationRatingEnumeration::SatireOrParodyContent => serializer
					.serialize_unit_variant(
						"MediaManipulationRatingEnumeration",
						3u32,
						"SatireOrParodyContent",
					),
				MediaManipulationRatingEnumeration::StagedContent => serializer
					.serialize_unit_variant(
						"MediaManipulationRatingEnumeration",
						4u32,
						"StagedContent",
					),
				MediaManipulationRatingEnumeration::TransformedContent => serializer
					.serialize_unit_variant(
						"MediaManipulationRatingEnumeration",
						5u32,
						"TransformedContent",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for MediaManipulationRatingEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				DecontextualizedContent,
				EditedOrCroppedContent,
				OriginalMediaContent,
				SatireOrParodyContent,
				StagedContent,
				TransformedContent,
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
						"DecontextualizedContent" => Ok(Field::DecontextualizedContent),
						"EditedOrCroppedContent" => Ok(Field::EditedOrCroppedContent),
						"OriginalMediaContent" => Ok(Field::OriginalMediaContent),
						"SatireOrParodyContent" => Ok(Field::SatireOrParodyContent),
						"StagedContent" => Ok(Field::StagedContent),
						"TransformedContent" => Ok(Field::TransformedContent),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"DecontextualizedContent" => Ok(Field::DecontextualizedContent),
						b"EditedOrCroppedContent" => Ok(Field::EditedOrCroppedContent),
						b"OriginalMediaContent" => Ok(Field::OriginalMediaContent),
						b"SatireOrParodyContent" => Ok(Field::SatireOrParodyContent),
						b"StagedContent" => Ok(Field::StagedContent),
						b"TransformedContent" => Ok(Field::TransformedContent),
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
				type Value = MediaManipulationRatingEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MediaManipulationRatingEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::DecontextualizedContent, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MediaManipulationRatingEnumeration::DecontextualizedContent)
						}
						(Field::EditedOrCroppedContent, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MediaManipulationRatingEnumeration::EditedOrCroppedContent)
						}
						(Field::OriginalMediaContent, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MediaManipulationRatingEnumeration::OriginalMediaContent)
						}
						(Field::SatireOrParodyContent, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MediaManipulationRatingEnumeration::SatireOrParodyContent)
						}
						(Field::StagedContent, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MediaManipulationRatingEnumeration::StagedContent)
						}
						(Field::TransformedContent, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MediaManipulationRatingEnumeration::TransformedContent)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"DecontextualizedContent",
				"EditedOrCroppedContent",
				"OriginalMediaContent",
				"SatireOrParodyContent",
				"StagedContent",
				"TransformedContent",
			];
			deserializer.deserialize_enum(
				"MediaManipulationRatingEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

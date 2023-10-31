/// <https://schema.org/BookFormatType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum BookFormatType {
	/// <https://schema.org/AudiobookFormat>
	AudiobookFormat,
	/// <https://schema.org/EBook>
	EBook,
	/// <https://schema.org/GraphicNovel>
	GraphicNovel,
	/// <https://schema.org/Hardcover>
	Hardcover,
	/// <https://schema.org/Paperback>
	Paperback,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for BookFormatType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				BookFormatType::AudiobookFormat => {
					serializer.serialize_unit_variant("BookFormatType", 0u32, "AudiobookFormat")
				}
				BookFormatType::EBook => {
					serializer.serialize_unit_variant("BookFormatType", 1u32, "EBook")
				}
				BookFormatType::GraphicNovel => {
					serializer.serialize_unit_variant("BookFormatType", 2u32, "GraphicNovel")
				}
				BookFormatType::Hardcover => {
					serializer.serialize_unit_variant("BookFormatType", 3u32, "Hardcover")
				}
				BookFormatType::Paperback => {
					serializer.serialize_unit_variant("BookFormatType", 4u32, "Paperback")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for BookFormatType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AudiobookFormat,
				EBook,
				GraphicNovel,
				Hardcover,
				Paperback,
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
						"AudiobookFormat" => Ok(Field::AudiobookFormat),
						"EBook" => Ok(Field::EBook),
						"GraphicNovel" => Ok(Field::GraphicNovel),
						"Hardcover" => Ok(Field::Hardcover),
						"Paperback" => Ok(Field::Paperback),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"AudiobookFormat" => Ok(Field::AudiobookFormat),
						b"EBook" => Ok(Field::EBook),
						b"GraphicNovel" => Ok(Field::GraphicNovel),
						b"Hardcover" => Ok(Field::Hardcover),
						b"Paperback" => Ok(Field::Paperback),
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
				type Value = BookFormatType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema BookFormatType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::AudiobookFormat, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BookFormatType::AudiobookFormat)
						}
						(Field::EBook, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BookFormatType::EBook)
						}
						(Field::GraphicNovel, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BookFormatType::GraphicNovel)
						}
						(Field::Hardcover, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BookFormatType::Hardcover)
						}
						(Field::Paperback, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(BookFormatType::Paperback)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"AudiobookFormat",
				"EBook",
				"GraphicNovel",
				"Hardcover",
				"Paperback",
			];
			deserializer.deserialize_enum("BookFormatType", VARIANTS, EnumerationVisitor)
		}
	}
}

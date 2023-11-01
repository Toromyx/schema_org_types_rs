/// <https://schema.org/DayOfWeek>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DayOfWeek {
	/// <https://schema.org/Friday>
	Friday,
	/// <https://schema.org/Monday>
	Monday,
	/// <https://schema.org/PublicHolidays>
	PublicHolidays,
	/// <https://schema.org/Saturday>
	Saturday,
	/// <https://schema.org/Sunday>
	Sunday,
	/// <https://schema.org/Thursday>
	Thursday,
	/// <https://schema.org/Tuesday>
	Tuesday,
	/// <https://schema.org/Wednesday>
	Wednesday,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DayOfWeek {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DayOfWeek::Friday => serializer.serialize_unit_variant("DayOfWeek", 0u32, "Friday"),
				DayOfWeek::Monday => serializer.serialize_unit_variant("DayOfWeek", 1u32, "Monday"),
				DayOfWeek::PublicHolidays => {
					serializer.serialize_unit_variant("DayOfWeek", 2u32, "PublicHolidays")
				}
				DayOfWeek::Saturday => {
					serializer.serialize_unit_variant("DayOfWeek", 3u32, "Saturday")
				}
				DayOfWeek::Sunday => serializer.serialize_unit_variant("DayOfWeek", 4u32, "Sunday"),
				DayOfWeek::Thursday => {
					serializer.serialize_unit_variant("DayOfWeek", 5u32, "Thursday")
				}
				DayOfWeek::Tuesday => {
					serializer.serialize_unit_variant("DayOfWeek", 6u32, "Tuesday")
				}
				DayOfWeek::Wednesday => {
					serializer.serialize_unit_variant("DayOfWeek", 7u32, "Wednesday")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for DayOfWeek {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Friday,
				Monday,
				PublicHolidays,
				Saturday,
				Sunday,
				Thursday,
				Tuesday,
				Wednesday,
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
						"Friday" => Ok(Field::Friday),
						"Monday" => Ok(Field::Monday),
						"PublicHolidays" => Ok(Field::PublicHolidays),
						"Saturday" => Ok(Field::Saturday),
						"Sunday" => Ok(Field::Sunday),
						"Thursday" => Ok(Field::Thursday),
						"Tuesday" => Ok(Field::Tuesday),
						"Wednesday" => Ok(Field::Wednesday),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Friday" => Ok(Field::Friday),
						b"Monday" => Ok(Field::Monday),
						b"PublicHolidays" => Ok(Field::PublicHolidays),
						b"Saturday" => Ok(Field::Saturday),
						b"Sunday" => Ok(Field::Sunday),
						b"Thursday" => Ok(Field::Thursday),
						b"Tuesday" => Ok(Field::Tuesday),
						b"Wednesday" => Ok(Field::Wednesday),
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
				type Value = DayOfWeek;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DayOfWeek")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Friday, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DayOfWeek::Friday)
						}
						(Field::Monday, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DayOfWeek::Monday)
						}
						(Field::PublicHolidays, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DayOfWeek::PublicHolidays)
						}
						(Field::Saturday, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DayOfWeek::Saturday)
						}
						(Field::Sunday, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DayOfWeek::Sunday)
						}
						(Field::Thursday, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DayOfWeek::Thursday)
						}
						(Field::Tuesday, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DayOfWeek::Tuesday)
						}
						(Field::Wednesday, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DayOfWeek::Wednesday)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"Friday",
				"Monday",
				"PublicHolidays",
				"Saturday",
				"Sunday",
				"Thursday",
				"Tuesday",
				"Wednesday",
			];
			deserializer.deserialize_enum("DayOfWeek", VARIANTS, EnumerationVisitor)
		}
	}
}

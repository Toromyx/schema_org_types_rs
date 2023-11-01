/// <https://schema.org/MapCategoryType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MapCategoryType {
	/// <https://schema.org/ParkingMap>
	ParkingMap,
	/// <https://schema.org/SeatingMap>
	SeatingMap,
	/// <https://schema.org/TransitMap>
	TransitMap,
	/// <https://schema.org/VenueMap>
	VenueMap,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MapCategoryType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MapCategoryType::ParkingMap => {
					serializer.serialize_unit_variant("MapCategoryType", 0u32, "ParkingMap")
				}
				MapCategoryType::SeatingMap => {
					serializer.serialize_unit_variant("MapCategoryType", 1u32, "SeatingMap")
				}
				MapCategoryType::TransitMap => {
					serializer.serialize_unit_variant("MapCategoryType", 2u32, "TransitMap")
				}
				MapCategoryType::VenueMap => {
					serializer.serialize_unit_variant("MapCategoryType", 3u32, "VenueMap")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for MapCategoryType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ParkingMap,
				SeatingMap,
				TransitMap,
				VenueMap,
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
						"ParkingMap" => Ok(Field::ParkingMap),
						"SeatingMap" => Ok(Field::SeatingMap),
						"TransitMap" => Ok(Field::TransitMap),
						"VenueMap" => Ok(Field::VenueMap),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ParkingMap" => Ok(Field::ParkingMap),
						b"SeatingMap" => Ok(Field::SeatingMap),
						b"TransitMap" => Ok(Field::TransitMap),
						b"VenueMap" => Ok(Field::VenueMap),
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
				type Value = MapCategoryType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema MapCategoryType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ParkingMap, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MapCategoryType::ParkingMap)
						}
						(Field::SeatingMap, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MapCategoryType::SeatingMap)
						}
						(Field::TransitMap, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MapCategoryType::TransitMap)
						}
						(Field::VenueMap, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(MapCategoryType::VenueMap)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["ParkingMap", "SeatingMap", "TransitMap", "VenueMap"];
			deserializer.deserialize_enum("MapCategoryType", VARIANTS, EnumerationVisitor)
		}
	}
}

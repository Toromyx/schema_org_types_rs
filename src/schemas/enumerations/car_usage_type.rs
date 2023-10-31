/// <https://schema.org/CarUsageType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum CarUsageType {
	/// <https://schema.org/DrivingSchoolVehicleUsage>
	DrivingSchoolVehicleUsage,
	/// <https://schema.org/RentalVehicleUsage>
	RentalVehicleUsage,
	/// <https://schema.org/TaxiVehicleUsage>
	TaxiVehicleUsage,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for CarUsageType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				CarUsageType::DrivingSchoolVehicleUsage => serializer.serialize_unit_variant(
					"CarUsageType",
					0u32,
					"DrivingSchoolVehicleUsage",
				),
				CarUsageType::RentalVehicleUsage => {
					serializer.serialize_unit_variant("CarUsageType", 1u32, "RentalVehicleUsage")
				}
				CarUsageType::TaxiVehicleUsage => {
					serializer.serialize_unit_variant("CarUsageType", 2u32, "TaxiVehicleUsage")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for CarUsageType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				DrivingSchoolVehicleUsage,
				RentalVehicleUsage,
				TaxiVehicleUsage,
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
						"DrivingSchoolVehicleUsage" => Ok(Field::DrivingSchoolVehicleUsage),
						"RentalVehicleUsage" => Ok(Field::RentalVehicleUsage),
						"TaxiVehicleUsage" => Ok(Field::TaxiVehicleUsage),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"DrivingSchoolVehicleUsage" => Ok(Field::DrivingSchoolVehicleUsage),
						b"RentalVehicleUsage" => Ok(Field::RentalVehicleUsage),
						b"TaxiVehicleUsage" => Ok(Field::TaxiVehicleUsage),
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
				type Value = CarUsageType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema CarUsageType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::DrivingSchoolVehicleUsage, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(CarUsageType::DrivingSchoolVehicleUsage)
						}
						(Field::RentalVehicleUsage, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(CarUsageType::RentalVehicleUsage)
						}
						(Field::TaxiVehicleUsage, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(CarUsageType::TaxiVehicleUsage)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"DrivingSchoolVehicleUsage",
				"RentalVehicleUsage",
				"TaxiVehicleUsage",
			];
			deserializer.deserialize_enum("CarUsageType", VARIANTS, EnumerationVisitor)
		}
	}
}

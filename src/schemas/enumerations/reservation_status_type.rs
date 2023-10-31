/// <https://schema.org/ReservationStatusType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ReservationStatusType {
	/// <https://schema.org/ReservationCancelled>
	ReservationCancelled,
	/// <https://schema.org/ReservationConfirmed>
	ReservationConfirmed,
	/// <https://schema.org/ReservationHold>
	ReservationHold,
	/// <https://schema.org/ReservationPending>
	ReservationPending,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ReservationStatusType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ReservationStatusType::ReservationCancelled => serializer.serialize_unit_variant(
					"ReservationStatusType",
					0u32,
					"ReservationCancelled",
				),
				ReservationStatusType::ReservationConfirmed => serializer.serialize_unit_variant(
					"ReservationStatusType",
					1u32,
					"ReservationConfirmed",
				),
				ReservationStatusType::ReservationHold => serializer.serialize_unit_variant(
					"ReservationStatusType",
					2u32,
					"ReservationHold",
				),
				ReservationStatusType::ReservationPending => serializer.serialize_unit_variant(
					"ReservationStatusType",
					3u32,
					"ReservationPending",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for ReservationStatusType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ReservationCancelled,
				ReservationConfirmed,
				ReservationHold,
				ReservationPending,
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
						"ReservationCancelled" => Ok(Field::ReservationCancelled),
						"ReservationConfirmed" => Ok(Field::ReservationConfirmed),
						"ReservationHold" => Ok(Field::ReservationHold),
						"ReservationPending" => Ok(Field::ReservationPending),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ReservationCancelled" => Ok(Field::ReservationCancelled),
						b"ReservationConfirmed" => Ok(Field::ReservationConfirmed),
						b"ReservationHold" => Ok(Field::ReservationHold),
						b"ReservationPending" => Ok(Field::ReservationPending),
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
				type Value = ReservationStatusType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ReservationStatusType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ReservationCancelled, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReservationStatusType::ReservationCancelled)
						}
						(Field::ReservationConfirmed, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReservationStatusType::ReservationConfirmed)
						}
						(Field::ReservationHold, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReservationStatusType::ReservationHold)
						}
						(Field::ReservationPending, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(ReservationStatusType::ReservationPending)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ReservationCancelled",
				"ReservationConfirmed",
				"ReservationHold",
				"ReservationPending",
			];
			deserializer.deserialize_enum("ReservationStatusType", VARIANTS, EnumerationVisitor)
		}
	}
}

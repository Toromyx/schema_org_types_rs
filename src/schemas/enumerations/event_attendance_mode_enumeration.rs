/// <https://schema.org/EventAttendanceModeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum EventAttendanceModeEnumeration {
	/// <https://schema.org/MixedEventAttendanceMode>
	MixedEventAttendanceMode,
	/// <https://schema.org/OfflineEventAttendanceMode>
	OfflineEventAttendanceMode,
	/// <https://schema.org/OnlineEventAttendanceMode>
	OnlineEventAttendanceMode,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for EventAttendanceModeEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				EventAttendanceModeEnumeration::MixedEventAttendanceMode => serializer
					.serialize_unit_variant(
						"EventAttendanceModeEnumeration",
						0u32,
						"MixedEventAttendanceMode",
					),
				EventAttendanceModeEnumeration::OfflineEventAttendanceMode => serializer
					.serialize_unit_variant(
						"EventAttendanceModeEnumeration",
						1u32,
						"OfflineEventAttendanceMode",
					),
				EventAttendanceModeEnumeration::OnlineEventAttendanceMode => serializer
					.serialize_unit_variant(
						"EventAttendanceModeEnumeration",
						2u32,
						"OnlineEventAttendanceMode",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for EventAttendanceModeEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				MixedEventAttendanceMode,
				OfflineEventAttendanceMode,
				OnlineEventAttendanceMode,
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
						"MixedEventAttendanceMode" => Ok(Field::MixedEventAttendanceMode),
						"OfflineEventAttendanceMode" => Ok(Field::OfflineEventAttendanceMode),
						"OnlineEventAttendanceMode" => Ok(Field::OnlineEventAttendanceMode),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"MixedEventAttendanceMode" => Ok(Field::MixedEventAttendanceMode),
						b"OfflineEventAttendanceMode" => Ok(Field::OfflineEventAttendanceMode),
						b"OnlineEventAttendanceMode" => Ok(Field::OnlineEventAttendanceMode),
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
				type Value = EventAttendanceModeEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema EventAttendanceModeEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::MixedEventAttendanceMode, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EventAttendanceModeEnumeration::MixedEventAttendanceMode)
						}
						(Field::OfflineEventAttendanceMode, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EventAttendanceModeEnumeration::OfflineEventAttendanceMode)
						}
						(Field::OnlineEventAttendanceMode, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EventAttendanceModeEnumeration::OnlineEventAttendanceMode)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"MixedEventAttendanceMode",
				"OfflineEventAttendanceMode",
				"OnlineEventAttendanceMode",
			];
			deserializer.deserialize_enum(
				"EventAttendanceModeEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}

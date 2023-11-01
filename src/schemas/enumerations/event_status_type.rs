/// <https://schema.org/EventStatusType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum EventStatusType {
	/// <https://schema.org/EventCancelled>
	EventCancelled,
	/// <https://schema.org/EventMovedOnline>
	EventMovedOnline,
	/// <https://schema.org/EventPostponed>
	EventPostponed,
	/// <https://schema.org/EventRescheduled>
	EventRescheduled,
	/// <https://schema.org/EventScheduled>
	EventScheduled,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for EventStatusType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				EventStatusType::EventCancelled => {
					serializer.serialize_unit_variant("EventStatusType", 0u32, "EventCancelled")
				}
				EventStatusType::EventMovedOnline => {
					serializer.serialize_unit_variant("EventStatusType", 1u32, "EventMovedOnline")
				}
				EventStatusType::EventPostponed => {
					serializer.serialize_unit_variant("EventStatusType", 2u32, "EventPostponed")
				}
				EventStatusType::EventRescheduled => {
					serializer.serialize_unit_variant("EventStatusType", 3u32, "EventRescheduled")
				}
				EventStatusType::EventScheduled => {
					serializer.serialize_unit_variant("EventStatusType", 4u32, "EventScheduled")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for EventStatusType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				EventCancelled,
				EventMovedOnline,
				EventPostponed,
				EventRescheduled,
				EventScheduled,
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
						"EventCancelled" => Ok(Field::EventCancelled),
						"EventMovedOnline" => Ok(Field::EventMovedOnline),
						"EventPostponed" => Ok(Field::EventPostponed),
						"EventRescheduled" => Ok(Field::EventRescheduled),
						"EventScheduled" => Ok(Field::EventScheduled),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"EventCancelled" => Ok(Field::EventCancelled),
						b"EventMovedOnline" => Ok(Field::EventMovedOnline),
						b"EventPostponed" => Ok(Field::EventPostponed),
						b"EventRescheduled" => Ok(Field::EventRescheduled),
						b"EventScheduled" => Ok(Field::EventScheduled),
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
				type Value = EventStatusType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema EventStatusType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::EventCancelled, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EventStatusType::EventCancelled)
						}
						(Field::EventMovedOnline, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EventStatusType::EventMovedOnline)
						}
						(Field::EventPostponed, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EventStatusType::EventPostponed)
						}
						(Field::EventRescheduled, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EventStatusType::EventRescheduled)
						}
						(Field::EventScheduled, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(EventStatusType::EventScheduled)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"EventCancelled",
				"EventMovedOnline",
				"EventPostponed",
				"EventRescheduled",
				"EventScheduled",
			];
			deserializer.deserialize_enum("EventStatusType", VARIANTS, EnumerationVisitor)
		}
	}
}

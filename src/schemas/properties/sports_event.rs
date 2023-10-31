use super::*;
/// <https://schema.org/sportsEvent>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum SportsEventProperty {
	#[cfg(any(
		any(feature = "sports-event-schema", feature = "general-schema-section"),
		doc
	))]
	SportsEvent(SportsEvent),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for SportsEventProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "sports-event-schema", feature = "general-schema-section"),
					doc
				))]
				SportsEventProperty::SportsEvent(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for SportsEventProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "sports-event-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<SportsEvent as Deserialize>::deserialize(deserializer),
				SportsEventProperty::SportsEvent,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property sportsEvent",
			))
		}
	}
}

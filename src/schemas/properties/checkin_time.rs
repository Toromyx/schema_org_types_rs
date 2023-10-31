use super::*;
/// <https://schema.org/checkinTime>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum CheckinTimeProperty {
	#[cfg(any(any(feature = "time-schema", feature = "general-schema-section"), doc))]
	Time(Time),
	#[cfg(any(
		any(feature = "date-time-schema", feature = "general-schema-section"),
		doc
	))]
	DateTime(DateTime),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for CheckinTimeProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(any(feature = "time-schema", feature = "general-schema-section"), doc))]
				CheckinTimeProperty::Time(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "date-time-schema", feature = "general-schema-section"),
					doc
				))]
				CheckinTimeProperty::DateTime(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for CheckinTimeProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(any(feature = "time-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Time as Deserialize>::deserialize(deserializer),
				CheckinTimeProperty::Time,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "date-time-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<DateTime as Deserialize>::deserialize(deserializer),
				CheckinTimeProperty::DateTime,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property checkinTime",
			))
		}
	}
}

use super::*;
/// <https://schema.org/availabilityStarts>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AvailabilityStartsProperty {
	/// <https://schema.org/Date>
	Date(Date),
	/// <https://schema.org/Time>
	Time(Time),
	/// <https://schema.org/DateTime>
	DateTime(DateTime),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AvailabilityStartsProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				AvailabilityStartsProperty::Date(ref inner) => inner.serialize(serializer),
				AvailabilityStartsProperty::Time(ref inner) => inner.serialize(serializer),
				AvailabilityStartsProperty::DateTime(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				AvailabilityStartsProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AvailabilityStartsProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<Date as Deserialize>::deserialize(deserializer),
				AvailabilityStartsProperty::Date,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Time as Deserialize>::deserialize(deserializer),
				AvailabilityStartsProperty::Time,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<DateTime as Deserialize>::deserialize(deserializer),
				AvailabilityStartsProperty::DateTime,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				AvailabilityStartsProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property availabilityStarts or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property availabilityStarts";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

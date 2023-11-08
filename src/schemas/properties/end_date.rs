use super::*;
/// <https://schema.org/endDate>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum EndDateProperty {
	#[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
	Date(Date),
	#[cfg(any(
		any(feature = "date-time-schema", feature = "general-schema-section"),
		doc
	))]
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
	impl Serialize for EndDateProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
				EndDateProperty::Date(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "date-time-schema", feature = "general-schema-section"),
					doc
				))]
				EndDateProperty::DateTime(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				EndDateProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for EndDateProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Date as Deserialize>::deserialize(deserializer),
				EndDateProperty::Date,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "date-time-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<DateTime as Deserialize>::deserialize(deserializer),
				EndDateProperty::DateTime,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				EndDateProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property endDate or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property endDate";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

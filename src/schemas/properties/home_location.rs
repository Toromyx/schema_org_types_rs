use super::*;
/// <https://schema.org/homeLocation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum HomeLocationProperty {
	#[cfg(any(
		any(feature = "contact-point-schema", feature = "general-schema-section"),
		doc
	))]
	ContactPoint(ContactPoint),
	#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
	Place(Place),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for HomeLocationProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "contact-point-schema", feature = "general-schema-section"),
					doc
				))]
				HomeLocationProperty::ContactPoint(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
				HomeLocationProperty::Place(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for HomeLocationProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "contact-point-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ContactPoint as Deserialize>::deserialize(deserializer),
				HomeLocationProperty::ContactPoint,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Place as Deserialize>::deserialize(deserializer),
				HomeLocationProperty::Place,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property homeLocation",
			))
		}
	}
}

use super::*;
/// <https://schema.org/location>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum LocationProperty {
	/// <https://schema.org/Place>
	Place(Place),
	/// <https://schema.org/PostalAddress>
	PostalAddress(PostalAddress),
	/// <https://schema.org/VirtualLocation>
	VirtualLocation(VirtualLocation),
	/// <https://schema.org/Text>
	Text(Text),
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
	impl Serialize for LocationProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				LocationProperty::Place(ref inner) => inner.serialize(serializer),
				LocationProperty::PostalAddress(ref inner) => inner.serialize(serializer),
				LocationProperty::VirtualLocation(ref inner) => inner.serialize(serializer),
				LocationProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				LocationProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for LocationProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<Place as Deserialize>::deserialize(deserializer),
				LocationProperty::Place,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<PostalAddress as Deserialize>::deserialize(deserializer),
				LocationProperty::PostalAddress,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<VirtualLocation as Deserialize>::deserialize(deserializer),
				LocationProperty::VirtualLocation,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				LocationProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				LocationProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property location or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property location";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

use super::*;
/// <https://schema.org/originalMediaLink>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum OriginalMediaLinkProperty {
	/// <https://schema.org/MediaObject>
	MediaObject(MediaObject),
	/// <https://schema.org/WebPage>
	WebPage(WebPage),
	/// <https://schema.org/URL>
	Url(Url),
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
	impl Serialize for OriginalMediaLinkProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				OriginalMediaLinkProperty::MediaObject(ref inner) => inner.serialize(serializer),
				OriginalMediaLinkProperty::WebPage(ref inner) => inner.serialize(serializer),
				OriginalMediaLinkProperty::Url(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				OriginalMediaLinkProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for OriginalMediaLinkProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<MediaObject as Deserialize>::deserialize(deserializer),
				OriginalMediaLinkProperty::MediaObject,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<WebPage as Deserialize>::deserialize(deserializer),
				OriginalMediaLinkProperty::WebPage,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				OriginalMediaLinkProperty::Url,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				OriginalMediaLinkProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property originalMediaLink or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property originalMediaLink";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

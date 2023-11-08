use super::*;
/// <https://schema.org/screenshot>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ScreenshotProperty {
	#[cfg(any(
		any(feature = "image-object-schema", feature = "general-schema-section"),
		doc
	))]
	ImageObject(ImageObject),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
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
	impl Serialize for ScreenshotProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "image-object-schema", feature = "general-schema-section"),
					doc
				))]
				ScreenshotProperty::ImageObject(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
				ScreenshotProperty::Url(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				ScreenshotProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ScreenshotProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "image-object-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ImageObject as Deserialize>::deserialize(deserializer),
				ScreenshotProperty::ImageObject,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				ScreenshotProperty::Url,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				ScreenshotProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property screenshot or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property screenshot";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

use super::*;
/// <https://schema.org/photos>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PhotosProperty {
	#[cfg(any(
		any(feature = "image-object-schema", feature = "general-schema-section"),
		doc
	))]
	ImageObject(ImageObject),
	#[cfg(any(
		any(feature = "photograph-schema", feature = "general-schema-section"),
		doc
	))]
	Photograph(Photograph),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for PhotosProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "image-object-schema", feature = "general-schema-section"),
					doc
				))]
				PhotosProperty::ImageObject(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "photograph-schema", feature = "general-schema-section"),
					doc
				))]
				PhotosProperty::Photograph(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for PhotosProperty {
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
				PhotosProperty::ImageObject,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "photograph-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Photograph as Deserialize>::deserialize(deserializer),
				PhotosProperty::Photograph,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property photos",
			))
		}
	}
}

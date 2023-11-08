use super::*;
/// <https://schema.org/audio>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AudioProperty {
	#[cfg(any(
		any(feature = "audio-object-schema", feature = "general-schema-section"),
		doc
	))]
	AudioObject(AudioObject),
	#[cfg(any(any(feature = "clip-schema", feature = "general-schema-section"), doc))]
	Clip(Clip),
	#[cfg(any(
		any(feature = "music-recording-schema", feature = "general-schema-section"),
		doc
	))]
	MusicRecording(MusicRecording),
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
	impl Serialize for AudioProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "audio-object-schema", feature = "general-schema-section"),
					doc
				))]
				AudioProperty::AudioObject(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "clip-schema", feature = "general-schema-section"), doc))]
				AudioProperty::Clip(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "music-recording-schema", feature = "general-schema-section"),
					doc
				))]
				AudioProperty::MusicRecording(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				AudioProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AudioProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "audio-object-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AudioObject as Deserialize>::deserialize(deserializer),
				AudioProperty::AudioObject,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "clip-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Clip as Deserialize>::deserialize(deserializer),
				AudioProperty::Clip,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "music-recording-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MusicRecording as Deserialize>::deserialize(deserializer),
				AudioProperty::MusicRecording,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				AudioProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property audio or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property audio";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

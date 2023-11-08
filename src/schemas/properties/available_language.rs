use super::*;
/// <https://schema.org/availableLanguage>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AvailableLanguageProperty {
	#[cfg(any(
		any(feature = "language-schema", feature = "general-schema-section"),
		doc
	))]
	Language(Language),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
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
	impl Serialize for AvailableLanguageProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "language-schema", feature = "general-schema-section"),
					doc
				))]
				AvailableLanguageProperty::Language(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				AvailableLanguageProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				AvailableLanguageProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AvailableLanguageProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "language-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Language as Deserialize>::deserialize(deserializer),
				AvailableLanguageProperty::Language,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				AvailableLanguageProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				AvailableLanguageProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property availableLanguage or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property availableLanguage";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

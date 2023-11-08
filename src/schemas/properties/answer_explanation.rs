use super::*;
/// <https://schema.org/answerExplanation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AnswerExplanationProperty {
	#[cfg(any(
		any(feature = "comment-schema", feature = "general-schema-section"),
		doc
	))]
	Comment(Comment),
	#[cfg(any(
		any(feature = "web-content-schema", feature = "pending-schema-section"),
		doc
	))]
	WebContent(WebContent),
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
	impl Serialize for AnswerExplanationProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "comment-schema", feature = "general-schema-section"),
					doc
				))]
				AnswerExplanationProperty::Comment(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "web-content-schema", feature = "pending-schema-section"),
					doc
				))]
				AnswerExplanationProperty::WebContent(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				AnswerExplanationProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AnswerExplanationProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "comment-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Comment as Deserialize>::deserialize(deserializer),
				AnswerExplanationProperty::Comment,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "web-content-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<WebContent as Deserialize>::deserialize(deserializer),
				AnswerExplanationProperty::WebContent,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				AnswerExplanationProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property answerExplanation or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property answerExplanation";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

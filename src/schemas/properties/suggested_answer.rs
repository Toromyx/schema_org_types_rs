use super::*;
/// <https://schema.org/suggestedAnswer>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum SuggestedAnswerProperty {
	#[cfg(any(
		any(feature = "answer-schema", feature = "general-schema-section"),
		doc
	))]
	Answer(Answer),
	#[cfg(any(
		any(feature = "item-list-schema", feature = "general-schema-section"),
		doc
	))]
	ItemList(ItemList),
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
	impl Serialize for SuggestedAnswerProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "answer-schema", feature = "general-schema-section"),
					doc
				))]
				SuggestedAnswerProperty::Answer(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "item-list-schema", feature = "general-schema-section"),
					doc
				))]
				SuggestedAnswerProperty::ItemList(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				SuggestedAnswerProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for SuggestedAnswerProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "answer-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Answer as Deserialize>::deserialize(deserializer),
				SuggestedAnswerProperty::Answer,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "item-list-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ItemList as Deserialize>::deserialize(deserializer),
				SuggestedAnswerProperty::ItemList,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				SuggestedAnswerProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property suggestedAnswer or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property suggestedAnswer";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

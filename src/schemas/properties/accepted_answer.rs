use super::*;
/// <https://schema.org/acceptedAnswer>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AcceptedAnswerProperty {
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
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AcceptedAnswerProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "answer-schema", feature = "general-schema-section"),
					doc
				))]
				AcceptedAnswerProperty::Answer(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "item-list-schema", feature = "general-schema-section"),
					doc
				))]
				AcceptedAnswerProperty::ItemList(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AcceptedAnswerProperty {
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
				AcceptedAnswerProperty::Answer,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "item-list-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ItemList as Deserialize>::deserialize(deserializer),
				AcceptedAnswerProperty::ItemList,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property acceptedAnswer",
			))
		}
	}
}

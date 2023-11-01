use super::*;
/// <https://schema.org/positiveNotes>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PositiveNotesProperty {
	#[cfg(any(
		any(feature = "item-list-schema", feature = "general-schema-section"),
		doc
	))]
	ItemList(ItemList),
	#[cfg(any(
		any(feature = "list-item-schema", feature = "general-schema-section"),
		doc
	))]
	ListItem(ListItem),
	#[cfg(any(
		any(feature = "web-content-schema", feature = "pending-schema-section"),
		doc
	))]
	WebContent(WebContent),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for PositiveNotesProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "item-list-schema", feature = "general-schema-section"),
					doc
				))]
				PositiveNotesProperty::ItemList(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "list-item-schema", feature = "general-schema-section"),
					doc
				))]
				PositiveNotesProperty::ListItem(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "web-content-schema", feature = "pending-schema-section"),
					doc
				))]
				PositiveNotesProperty::WebContent(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				PositiveNotesProperty::Text(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for PositiveNotesProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "item-list-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ItemList as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::ItemList,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "list-item-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ListItem as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::ListItem,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "web-content-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<WebContent as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::WebContent,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::Text,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property positiveNotes",
			))
		}
	}
}

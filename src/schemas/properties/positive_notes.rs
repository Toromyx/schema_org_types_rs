use super::*;
/// <https://schema.org/positiveNotes>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PositiveNotesProperty {
	/// <https://schema.org/ItemList>
	ItemList(ItemList),
	/// <https://schema.org/ListItem>
	ListItem(ListItem),
	/// <https://schema.org/WebContent>
	WebContent(WebContent),
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
	impl Serialize for PositiveNotesProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PositiveNotesProperty::ItemList(ref inner) => inner.serialize(serializer),
				PositiveNotesProperty::ListItem(ref inner) => inner.serialize(serializer),
				PositiveNotesProperty::WebContent(ref inner) => inner.serialize(serializer),
				PositiveNotesProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				PositiveNotesProperty::SerdeFail(ref inner) => inner.serialize(serializer),
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
			if let Ok(ok) = Result::map(
				<ItemList as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::ItemList,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<ListItem as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::ListItem,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<WebContent as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::WebContent,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				PositiveNotesProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property positiveNotes or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property positiveNotes";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}

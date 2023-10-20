use super::*;
/// <https://schema.org/dataFeedElement>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataFeedElementProperty {
	#[cfg(any(
		any(feature = "data-feed-item-schema", feature = "general-schema-section"),
		doc
	))]
	DataFeedItem(DataFeedItem),
	#[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
	Thing(Thing),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}

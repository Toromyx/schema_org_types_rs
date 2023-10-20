use super::*;
/// <https://schema.org/webFeed>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WebFeedProperty {
	#[cfg(any(
		any(feature = "data-feed-schema", feature = "general-schema-section"),
		doc
	))]
	DataFeed(DataFeed),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
}

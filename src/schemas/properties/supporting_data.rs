use super::*;
/// <https://schema.org/supportingData>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SupportingDataProperty {
	#[cfg(any(
		any(feature = "data-feed-schema", feature = "general-schema-section"),
		doc
	))]
	DataFeed(DataFeed),
}

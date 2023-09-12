use super::*;
/// An item within a data feed. Data feeds may have many elements.
///
/// https://schema.org/dataFeedElement
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataFeedElementProperty {
    #[cfg(any(feature = "data-feed-item-schema", feature = "general-schema-section"))]
    DataFeedItem(DataFeedItem),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "thing-schema", feature = "general-schema-section"))]
    Thing(Thing),
}

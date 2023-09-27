use super::*;
/// <https://schema.org/webFeed>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum WebFeedProperty {
    #[cfg(any(
        any(feature = "data-feed-schema", feature = "general-schema-section"),
        doc
    ))]
    DataFeed(DataFeed),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}

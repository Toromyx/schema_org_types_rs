use super::*;
/// The URL for a feed, e.g. associated with a podcast series, blog, or series of date-stamped updates. This is usually RSS or Atom.
///
/// https://schema.org/webFeed
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WebFeedProperty {
    #[cfg(any(feature = "data-feed-schema", feature = "general-schema-section"))]
    DataFeed(DataFeed),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}

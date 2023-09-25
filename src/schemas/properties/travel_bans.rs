use super::*;
/// Information about travel bans, e.g. in the context of a pandemic.
///
/// <https://schema.org/travelBans>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TravelBansProperty {
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
    #[cfg(any(
        any(feature = "web-content-schema", feature = "pending-schema-section"),
        doc
    ))]
    WebContent(WebContent),
}

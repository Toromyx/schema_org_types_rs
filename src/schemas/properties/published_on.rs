use super::*;
/// A broadcast service associated with the publication event.
///
/// <https://schema.org/publishedOn>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PublishedOnProperty {
    #[cfg(any(
        any(
            feature = "broadcast-service-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BroadcastService(BroadcastService),
}

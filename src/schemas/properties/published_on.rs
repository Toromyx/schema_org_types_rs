use super::*;
/// A broadcast service associated with the publication event.
///
/// https://schema.org/publishedOn
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PublishedOnProperty {
    #[cfg(any(
        feature = "broadcast-service-schema",
        feature = "general-schema-section"
    ))]
    BroadcastService(BroadcastService),
}

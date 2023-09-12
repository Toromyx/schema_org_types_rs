use super::*;
/// The place and time the release was issued, expressed as a PublicationEvent.
///
/// https://schema.org/releasedEvent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReleasedEventProperty {
    #[cfg(any(
        feature = "publication-event-schema",
        feature = "general-schema-section"
    ))]
    PublicationEvent(PublicationEvent),
}

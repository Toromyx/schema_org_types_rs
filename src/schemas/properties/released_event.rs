use super::*;
/// <https://schema.org/releasedEvent>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ReleasedEventProperty {
    #[cfg(any(
        any(
            feature = "publication-event-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    PublicationEvent(PublicationEvent),
}

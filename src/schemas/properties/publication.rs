use super::*;
/// A publication event associated with the item.
///
/// https://schema.org/publication
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PublicationProperty {
    #[cfg(any(
        feature = "publication-event-schema",
        feature = "general-schema-section"
    ))]
    PublicationEvent(PublicationEvent),
}

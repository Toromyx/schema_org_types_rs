use super::*;
/// An episode of a TV/radio series or season.
///
/// <https://schema.org/episodes>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EpisodesProperty {
    #[cfg(any(
        any(feature = "episode-schema", feature = "general-schema-section"),
        doc
    ))]
    Episode(Episode),
}

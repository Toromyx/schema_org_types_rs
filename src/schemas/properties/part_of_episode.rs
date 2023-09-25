use super::*;
/// The episode to which this clip belongs.
///
/// <https://schema.org/partOfEpisode>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PartOfEpisodeProperty {
    #[cfg(any(
        any(feature = "episode-schema", feature = "general-schema-section"),
        doc
    ))]
    Episode(Episode),
}

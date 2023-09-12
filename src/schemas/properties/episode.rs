use super::*;
/// An episode of a TV, radio or game media within a series or season.
///
/// https://schema.org/episode
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EpisodeProperty {
    #[cfg(any(feature = "episode-schema", feature = "general-schema-section"))]
    Episode(Episode),
}

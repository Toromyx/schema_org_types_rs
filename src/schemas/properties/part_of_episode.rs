use super::*;
/// The episode to which this clip belongs.
///
/// https://schema.org/partOfEpisode
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfEpisodeProperty {
    #[cfg(any(feature = "episode-schema", feature = "general-schema-section"))]
    Episode(Episode),
}

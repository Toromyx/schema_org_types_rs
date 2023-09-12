use super::*;
/// Position of the episode within an ordered group of episodes.
///
/// https://schema.org/episodeNumber
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EpisodeNumberProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}

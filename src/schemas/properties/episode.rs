use super::*;
/// <https://schema.org/episode>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EpisodeProperty {
	#[cfg(any(
		any(feature = "episode-schema", feature = "general-schema-section"),
		doc
	))]
	Episode(Episode),
}

use super::*;
/// <https://schema.org/numberOfEpisodes>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NumberOfEpisodesProperty {
	#[cfg(any(
		any(feature = "integer-schema", feature = "general-schema-section"),
		doc
	))]
	Integer(Integer),
}

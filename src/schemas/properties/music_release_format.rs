use super::*;
/// <https://schema.org/musicReleaseFormat>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MusicReleaseFormatProperty {
	#[cfg(any(
		any(
			feature = "music-release-format-type-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	MusicReleaseFormatType(MusicReleaseFormatType),
}

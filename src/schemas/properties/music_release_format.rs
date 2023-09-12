use super::*;
/// Format of this release (the type of recording media used, i.e. compact disc, digital media, LP, etc.).
///
/// https://schema.org/musicReleaseFormat
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MusicReleaseFormatProperty {
    #[cfg(any(
        feature = "music-release-format-type-schema",
        feature = "general-schema-section"
    ))]
    MusicReleaseFormatType(MusicReleaseFormatType),
}

use super::*;
/// Format of this release (the type of recording media used, i.e. compact disc, digital media, LP, etc.).
///
/// https://schema.org/musicReleaseFormat
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

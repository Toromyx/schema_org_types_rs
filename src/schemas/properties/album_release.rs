use super::*;
/// <https://schema.org/albumRelease>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AlbumReleaseProperty {
    #[cfg(any(
        any(feature = "music-release-schema", feature = "general-schema-section"),
        doc
    ))]
    MusicRelease(MusicRelease),
}

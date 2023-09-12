use super::*;
/// A release of this album.
///
/// https://schema.org/albumRelease
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AlbumReleaseProperty {
    #[cfg(any(feature = "music-release-schema", feature = "general-schema-section"))]
    MusicRelease(MusicRelease),
}

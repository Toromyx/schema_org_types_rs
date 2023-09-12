use super::*;
/// Classification of the album by its type of content: soundtrack, live album, studio album, etc.
///
/// https://schema.org/albumProductionType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AlbumProductionTypeProperty {
    #[cfg(any(
        feature = "music-album-production-type-schema",
        feature = "general-schema-section"
    ))]
    MusicAlbumProductionType(MusicAlbumProductionType),
}

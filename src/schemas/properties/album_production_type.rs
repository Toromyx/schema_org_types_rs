use super::*;
/// <https://schema.org/albumProductionType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AlbumProductionTypeProperty {
    #[cfg(any(
        any(
            feature = "music-album-production-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    MusicAlbumProductionType(MusicAlbumProductionType),
}

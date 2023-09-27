use super::*;
/// <https://schema.org/albumProductionType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

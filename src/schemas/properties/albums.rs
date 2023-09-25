use super::*;
/// A collection of music albums.
///
/// <https://schema.org/albums>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AlbumsProperty {
    #[cfg(any(
        any(feature = "music-album-schema", feature = "general-schema-section"),
        doc
    ))]
    MusicAlbum(MusicAlbum),
}

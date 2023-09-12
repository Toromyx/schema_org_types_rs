use super::*;
/// A collection of music albums.
///
/// https://schema.org/albums
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AlbumsProperty {
    #[cfg(any(feature = "music-album-schema", feature = "general-schema-section"))]
    MusicAlbum(MusicAlbum),
}

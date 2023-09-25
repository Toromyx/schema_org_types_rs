use super::*;
/// The playlist to which this recording belongs.
///
/// <https://schema.org/inPlaylist>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InPlaylistProperty {
    #[cfg(any(
        any(feature = "music-playlist-schema", feature = "general-schema-section"),
        doc
    ))]
    MusicPlaylist(MusicPlaylist),
}

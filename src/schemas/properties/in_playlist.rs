use super::*;
/// The playlist to which this recording belongs.
///
/// https://schema.org/inPlaylist
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InPlaylistProperty {
    #[cfg(any(feature = "music-playlist-schema", feature = "general-schema-section"))]
    MusicPlaylist(MusicPlaylist),
}
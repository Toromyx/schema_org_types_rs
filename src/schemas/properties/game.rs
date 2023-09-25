use super::*;
/// Video game which is played on this server.
///
/// https://schema.org/game
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GameProperty {
    #[cfg(any(
        any(feature = "video-game-schema", feature = "general-schema-section"),
        doc
    ))]
    VideoGame(VideoGame),
}

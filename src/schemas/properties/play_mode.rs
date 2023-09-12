use super::*;
/// Indicates whether this game is multi-player, co-op or single-player.  The game can be marked as multi-player, co-op and single-player at the same time.
///
/// https://schema.org/playMode
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PlayModeProperty {
    #[cfg(any(feature = "game-play-mode-schema", feature = "general-schema-section"))]
    GamePlayMode(GamePlayMode),
}

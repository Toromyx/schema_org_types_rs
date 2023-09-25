use super::*;
/// Indicates whether this game is multi-player, co-op or single-player.  The game can be marked as multi-player, co-op and single-player at the same time.
///
/// <https://schema.org/playMode>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PlayModeProperty {
    #[cfg(any(
        any(feature = "game-play-mode-schema", feature = "general-schema-section"),
        doc
    ))]
    GamePlayMode(GamePlayMode),
}

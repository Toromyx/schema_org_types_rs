use super::*;
/// <https://schema.org/game>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GameProperty {
    #[cfg(any(
        any(feature = "video-game-schema", feature = "general-schema-section"),
        doc
    ))]
    VideoGame(VideoGame),
}

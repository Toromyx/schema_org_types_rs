use super::*;
/// <https://schema.org/gameServer>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GameServerProperty {
    #[cfg(any(
        any(feature = "game-server-schema", feature = "general-schema-section"),
        doc
    ))]
    GameServer(GameServer),
}

use super::*;
/// The server on which  it is possible to play the game.
///
/// https://schema.org/gameServer
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GameServerProperty {
    #[cfg(any(feature = "game-server-schema", feature = "general-schema-section"))]
    GameServer(GameServer),
}

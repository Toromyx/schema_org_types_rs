use super::*;
/// Status of a game server.
///
/// https://schema.org/serverStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ServerStatusProperty {
    #[cfg(any(
        feature = "game-server-status-schema",
        feature = "general-schema-section"
    ))]
    GameServerStatus(GameServerStatus),
}

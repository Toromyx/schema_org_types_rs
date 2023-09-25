use super::*;
/// Status of a game server.
///
/// <https://schema.org/serverStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ServerStatusProperty {
    #[cfg(any(
        any(
            feature = "game-server-status-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    GameServerStatus(GameServerStatus),
}

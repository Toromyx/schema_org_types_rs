use super::*;
/// <https://schema.org/serverStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

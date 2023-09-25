/// Status of a game server.
///
/// https://schema.org/GameServerStatus
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GameServerStatus {
    /// Game server status: OfflinePermanently. Server is offline and not available.
    ///
    /// https://schema.org/OfflinePermanently
    OfflinePermanently,
    /// Game server status: OfflineTemporarily. Server is offline now but it can be online soon.
    ///
    /// https://schema.org/OfflineTemporarily
    OfflineTemporarily,
    /// Game server status: Online. Server is available.
    ///
    /// https://schema.org/Online
    Online,
    /// Game server status: OnlineFull. Server is online but unavailable. The maximum number of players has reached.
    ///
    /// https://schema.org/OnlineFull
    OnlineFull,
}

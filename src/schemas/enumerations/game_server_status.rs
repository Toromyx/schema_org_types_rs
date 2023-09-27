/// <https://schema.org/GameServerStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum GameServerStatus {
    /// <https://schema.org/OfflinePermanently>
    OfflinePermanently,
    /// <https://schema.org/OfflineTemporarily>
    OfflineTemporarily,
    /// <https://schema.org/Online>
    Online,
    /// <https://schema.org/OnlineFull>
    OnlineFull,
}

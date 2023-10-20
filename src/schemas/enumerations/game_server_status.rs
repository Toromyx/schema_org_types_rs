/// <https://schema.org/GameServerStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// <https://schema.org/MusicAlbumReleaseType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MusicAlbumReleaseType {
	/// <https://schema.org/AlbumRelease>
	AlbumRelease,
	/// <https://schema.org/BroadcastRelease>
	BroadcastRelease,
	/// <https://schema.org/EPRelease>
	EpRelease,
	/// <https://schema.org/SingleRelease>
	SingleRelease,
}

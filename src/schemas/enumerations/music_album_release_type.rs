/// The kind of release which this album is: single, EP or album.
///
/// https://schema.org/MusicAlbumReleaseType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MusicAlbumReleaseType {
    /// AlbumRelease.
    ///
    /// https://schema.org/AlbumRelease
    AlbumRelease,
    /// BroadcastRelease.
    ///
    /// https://schema.org/BroadcastRelease
    BroadcastRelease,
    /// EPRelease.
    ///
    /// https://schema.org/EPRelease
    EpRelease,
    /// SingleRelease.
    ///
    /// https://schema.org/SingleRelease
    SingleRelease,
}

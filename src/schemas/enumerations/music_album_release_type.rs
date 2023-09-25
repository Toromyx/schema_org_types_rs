/// The kind of release which this album is: single, EP or album.
///
/// https://schema.org/MusicAlbumReleaseType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

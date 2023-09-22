/// Classification of the album by its type of content: soundtrack, live album, studio album, etc.
///
/// https://schema.org/MusicAlbumProductionType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MusicAlbumProductionType {
    /// CompilationAlbum.
    ///
    /// https://schema.org/CompilationAlbum
    CompilationAlbum,
    /// DJMixAlbum.
    ///
    /// https://schema.org/DJMixAlbum
    DjMixAlbum,
    /// DemoAlbum.
    ///
    /// https://schema.org/DemoAlbum
    DemoAlbum,
    /// LiveAlbum.
    ///
    /// https://schema.org/LiveAlbum
    LiveAlbum,
    /// MixtapeAlbum.
    ///
    /// https://schema.org/MixtapeAlbum
    MixtapeAlbum,
    /// RemixAlbum.
    ///
    /// https://schema.org/RemixAlbum
    RemixAlbum,
    /// SoundtrackAlbum.
    ///
    /// https://schema.org/SoundtrackAlbum
    SoundtrackAlbum,
    /// SpokenWordAlbum.
    ///
    /// https://schema.org/SpokenWordAlbum
    SpokenWordAlbum,
    /// StudioAlbum.
    ///
    /// https://schema.org/StudioAlbum
    StudioAlbum,
}
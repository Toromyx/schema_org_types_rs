/// <https://schema.org/MusicAlbumProductionType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MusicAlbumProductionType {
    /// <https://schema.org/CompilationAlbum>
    CompilationAlbum,
    /// <https://schema.org/DJMixAlbum>
    DjMixAlbum,
    /// <https://schema.org/DemoAlbum>
    DemoAlbum,
    /// <https://schema.org/LiveAlbum>
    LiveAlbum,
    /// <https://schema.org/MixtapeAlbum>
    MixtapeAlbum,
    /// <https://schema.org/RemixAlbum>
    RemixAlbum,
    /// <https://schema.org/SoundtrackAlbum>
    SoundtrackAlbum,
    /// <https://schema.org/SpokenWordAlbum>
    SpokenWordAlbum,
    /// <https://schema.org/StudioAlbum>
    StudioAlbum,
}

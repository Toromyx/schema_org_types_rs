/// <https://schema.org/MusicAlbumProductionType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

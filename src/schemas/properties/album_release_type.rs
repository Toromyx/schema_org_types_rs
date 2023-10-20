use super::*;
/// <https://schema.org/albumReleaseType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AlbumReleaseTypeProperty {
	#[cfg(any(
		any(
			feature = "music-album-release-type-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	MusicAlbumReleaseType(MusicAlbumReleaseType),
}

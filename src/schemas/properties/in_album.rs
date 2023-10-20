use super::*;
/// <https://schema.org/inAlbum>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InAlbumProperty {
	#[cfg(any(
		any(feature = "music-album-schema", feature = "general-schema-section"),
		doc
	))]
	MusicAlbum(MusicAlbum),
}

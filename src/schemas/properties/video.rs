use super::*;
/// <https://schema.org/video>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VideoProperty {
	#[cfg(any(any(feature = "clip-schema", feature = "general-schema-section"), doc))]
	Clip(Clip),
	#[cfg(any(
		any(feature = "video-object-schema", feature = "general-schema-section"),
		doc
	))]
	VideoObject(VideoObject),
}

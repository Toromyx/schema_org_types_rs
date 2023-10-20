use super::*;
/// <https://schema.org/audio>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AudioProperty {
	#[cfg(any(
		any(feature = "audio-object-schema", feature = "general-schema-section"),
		doc
	))]
	AudioObject(AudioObject),
	#[cfg(any(any(feature = "clip-schema", feature = "general-schema-section"), doc))]
	Clip(Clip),
	#[cfg(any(
		any(feature = "music-recording-schema", feature = "general-schema-section"),
		doc
	))]
	MusicRecording(MusicRecording),
}

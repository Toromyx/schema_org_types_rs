use super::*;
/// An embedded audio object.
///
/// https://schema.org/audio
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AudioProperty {
    #[cfg(any(feature = "audio-object-schema", feature = "general-schema-section"))]
    AudioObject(AudioObject),
    #[cfg(any(feature = "clip-schema", feature = "general-schema-section"))]
    Clip(Clip),
    #[cfg(any(feature = "music-recording-schema", feature = "general-schema-section"))]
    MusicRecording(MusicRecording),
}

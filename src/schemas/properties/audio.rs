use super::*;
/// An embedded audio object.
///
/// <https://schema.org/audio>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

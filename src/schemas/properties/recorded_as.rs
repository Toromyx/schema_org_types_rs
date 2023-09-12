use super::*;
/// An audio recording of the work.
///
/// https://schema.org/recordedAs
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecordedAsProperty {
    #[cfg(any(feature = "music-recording-schema", feature = "general-schema-section"))]
    MusicRecording(MusicRecording),
}

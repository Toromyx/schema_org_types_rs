use super::*;
/// The composition this track is a recording of.
///
/// https://schema.org/recordingOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecordingOfProperty {
    #[cfg(any(
        feature = "music-composition-schema",
        feature = "general-schema-section"
    ))]
    MusicComposition(MusicComposition),
}

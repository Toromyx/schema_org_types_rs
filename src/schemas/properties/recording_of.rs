use super::*;
/// The composition this track is a recording of.
///
/// <https://schema.org/recordingOf>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RecordingOfProperty {
    #[cfg(any(
        any(
            feature = "music-composition-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    MusicComposition(MusicComposition),
}

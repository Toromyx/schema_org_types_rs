use super::*;
/// A music recording (track)&#x2014;usually a single song.
///
/// https://schema.org/tracks
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TracksProperty {
    #[cfg(any(feature = "music-recording-schema", feature = "general-schema-section"))]
    MusicRecording(MusicRecording),
}

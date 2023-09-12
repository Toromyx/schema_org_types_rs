use super::*;
/// The trailer of a movie or TV/radio series, season, episode, etc.
///
/// https://schema.org/trailer
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TrailerProperty {
    #[cfg(any(feature = "video-object-schema", feature = "general-schema-section"))]
    VideoObject(VideoObject),
}

use super::*;
/// An embedded video object.
///
/// https://schema.org/video
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VideoProperty {
    #[cfg(any(feature = "clip-schema", feature = "general-schema-section"))]
    Clip(Clip),
    #[cfg(any(feature = "video-object-schema", feature = "general-schema-section"))]
    VideoObject(VideoObject),
}

use super::*;
/// An embedded video object.
///
/// <https://schema.org/video>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum VideoProperty {
    #[cfg(any(any(feature = "clip-schema", feature = "general-schema-section"), doc))]
    Clip(Clip),
    #[cfg(any(
        any(feature = "video-object-schema", feature = "general-schema-section"),
        doc
    ))]
    VideoObject(VideoObject),
}

use super::*;
/// <https://schema.org/trailer>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TrailerProperty {
    #[cfg(any(
        any(feature = "video-object-schema", feature = "general-schema-section"),
        doc
    ))]
    VideoObject(VideoObject),
}

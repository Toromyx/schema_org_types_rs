use super::*;
/// Thumbnail image for an image or video.
///
/// https://schema.org/thumbnail
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ThumbnailProperty {
    #[cfg(any(
        any(feature = "image-object-schema", feature = "general-schema-section"),
        doc
    ))]
    ImageObject(ImageObject),
}

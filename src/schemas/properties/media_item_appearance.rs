use super::*;
/// In the context of a [[MediaReview]], indicates specific media item(s) that are grouped using a [[MediaReviewItem]].
///
/// https://schema.org/mediaItemAppearance
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MediaItemAppearanceProperty {
    #[cfg(any(feature = "media-object-schema", feature = "general-schema-section"))]
    MediaObject(MediaObject),
}

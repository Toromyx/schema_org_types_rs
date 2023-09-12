use super::*;
/// Indicates a MediaManipulationRatingEnumeration classification of a media object (in the context of how it was published or shared).
///
/// https://schema.org/mediaAuthenticityCategory
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MediaAuthenticityCategoryProperty {
    #[cfg(any(
        feature = "media-manipulation-rating-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    MediaManipulationRatingEnumeration(MediaManipulationRatingEnumeration),
}

use super::*;
/// <https://schema.org/mediaAuthenticityCategory>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MediaAuthenticityCategoryProperty {
    #[cfg(any(
        any(
            feature = "media-manipulation-rating-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    MediaManipulationRatingEnumeration(MediaManipulationRatingEnumeration),
}

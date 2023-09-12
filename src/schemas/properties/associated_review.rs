use super::*;
/// An associated [[Review]].
///
/// https://schema.org/associatedReview
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssociatedReviewProperty {
    #[cfg(any(feature = "review-schema", feature = "general-schema-section"))]
    Review(Review),
}

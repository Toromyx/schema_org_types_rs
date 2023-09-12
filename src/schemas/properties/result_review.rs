use super::*;
/// A sub property of result. The review that resulted in the performing of the action.
///
/// https://schema.org/resultReview
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ResultReviewProperty {
    #[cfg(any(feature = "review-schema", feature = "general-schema-section"))]
    Review(Review),
}

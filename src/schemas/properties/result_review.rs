use super::*;
/// A sub property of result. The review that resulted in the performing of the action.
///
/// <https://schema.org/resultReview>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ResultReviewProperty {
    #[cfg(any(
        any(feature = "review-schema", feature = "general-schema-section"),
        doc
    ))]
    Review(Review),
}

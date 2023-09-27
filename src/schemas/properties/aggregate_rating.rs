use super::*;
/// <https://schema.org/aggregateRating>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AggregateRatingProperty {
    #[cfg(any(
        any(
            feature = "aggregate-rating-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    AggregateRating(AggregateRating),
}

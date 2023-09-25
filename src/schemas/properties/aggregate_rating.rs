use super::*;
/// The overall rating, based on a collection of reviews or ratings, of the item.
///
/// https://schema.org/aggregateRating
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

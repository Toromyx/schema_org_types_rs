use super::*;
/// Official rating of a piece of content&#x2014;for example, 'MPAA PG-13'.
///
/// https://schema.org/contentRating
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ContentRatingProperty {
    #[cfg(any(
        any(feature = "rating-schema", feature = "general-schema-section"),
        doc
    ))]
    Rating(Rating),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}

use super::*;
/// Official rating of a piece of content&#x2014;for example, 'MPAA PG-13'.
///
/// https://schema.org/contentRating
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ContentRatingProperty {
    #[cfg(any(feature = "rating-schema", feature = "general-schema-section"))]
    Rating(Rating),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}

use super::*;
/// Date on which the content on this web page was last reviewed for accuracy and/or completeness.
///
/// https://schema.org/lastReviewed
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LastReviewedProperty {
    #[cfg(any(feature = "date-schema", feature = "general-schema-section"))]
    Date(Date),
}

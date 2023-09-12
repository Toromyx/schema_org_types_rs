use super::*;
/// A review of the item.
///
/// https://schema.org/review
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReviewProperty {
    #[cfg(any(feature = "review-schema", feature = "general-schema-section"))]
    Review(Review),
}

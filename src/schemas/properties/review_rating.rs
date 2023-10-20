use super::*;
/// <https://schema.org/reviewRating>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReviewRatingProperty {
	#[cfg(any(
		any(feature = "rating-schema", feature = "general-schema-section"),
		doc
	))]
	Rating(Rating),
}

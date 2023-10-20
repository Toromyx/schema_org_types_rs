use super::*;
/// <https://schema.org/itemCondition>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemConditionProperty {
	#[cfg(any(
		any(
			feature = "offer-item-condition-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	OfferItemCondition(OfferItemCondition),
}

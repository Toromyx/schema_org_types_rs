use super::*;
/// A predefined value from OfferItemCondition specifying the condition of the product or service, or the products or services included in the offer. Also used for product return policies to specify the condition of products accepted for returns.
///
/// https://schema.org/itemCondition
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemConditionProperty {
    #[cfg(any(
        feature = "offer-item-condition-schema",
        feature = "general-schema-section"
    ))]
    OfferItemCondition(OfferItemCondition),
}

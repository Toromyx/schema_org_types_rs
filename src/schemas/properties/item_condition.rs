use super::*;
/// A predefined value from OfferItemCondition specifying the condition of the product or service, or the products or services included in the offer. Also used for product return policies to specify the condition of products accepted for returns.
///
/// <https://schema.org/itemCondition>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

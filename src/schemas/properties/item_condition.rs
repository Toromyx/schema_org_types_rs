use super::*;
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

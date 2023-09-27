use super::*;
/// <https://schema.org/appliesToDeliveryMethod>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AppliesToDeliveryMethodProperty {
    #[cfg(any(
        any(feature = "delivery-method-schema", feature = "general-schema-section"),
        doc
    ))]
    DeliveryMethod(DeliveryMethod),
}

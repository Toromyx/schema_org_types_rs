use super::*;
/// The delivery method(s) to which the delivery charge or payment charge specification applies.
///
/// <https://schema.org/appliesToDeliveryMethod>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AppliesToDeliveryMethodProperty {
    #[cfg(any(
        any(feature = "delivery-method-schema", feature = "general-schema-section"),
        doc
    ))]
    DeliveryMethod(DeliveryMethod),
}

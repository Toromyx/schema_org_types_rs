use super::*;
/// A sub property of instrument. The method of delivery.
///
/// https://schema.org/deliveryMethod
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DeliveryMethodProperty {
    #[cfg(any(
        any(feature = "delivery-method-schema", feature = "general-schema-section"),
        doc
    ))]
    DeliveryMethod(DeliveryMethod),
}

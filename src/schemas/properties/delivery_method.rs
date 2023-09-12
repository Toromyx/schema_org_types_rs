use super::*;
/// A sub property of instrument. The method of delivery.
///
/// https://schema.org/deliveryMethod
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DeliveryMethodProperty {
    #[cfg(any(feature = "delivery-method-schema", feature = "general-schema-section"))]
    DeliveryMethod(DeliveryMethod),
}

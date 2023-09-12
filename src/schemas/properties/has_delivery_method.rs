use super::*;
/// Method used for delivery or shipping.
///
/// https://schema.org/hasDeliveryMethod
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasDeliveryMethodProperty {
    #[cfg(any(feature = "delivery-method-schema", feature = "general-schema-section"))]
    DeliveryMethod(DeliveryMethod),
}

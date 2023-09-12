use super::*;
/// The total delay between the receipt of the order and the goods reaching the final customer.
///
/// https://schema.org/deliveryTime
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DeliveryTimeProperty {
    #[cfg(any(
        feature = "shipping-delivery-time-schema",
        feature = "pending-schema-section"
    ))]
    ShippingDeliveryTime(ShippingDeliveryTime),
}

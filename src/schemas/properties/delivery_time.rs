use super::*;
/// The total delay between the receipt of the order and the goods reaching the final customer.
///
/// <https://schema.org/deliveryTime>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DeliveryTimeProperty {
    #[cfg(any(
        any(
            feature = "shipping-delivery-time-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    ShippingDeliveryTime(ShippingDeliveryTime),
}

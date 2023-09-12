use super::*;
/// New entry added as the package passes through each leg of its journey (from shipment to final delivery).
///
/// https://schema.org/deliveryStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DeliveryStatusProperty {
    #[cfg(any(feature = "delivery-event-schema", feature = "general-schema-section"))]
    DeliveryEvent(DeliveryEvent),
}

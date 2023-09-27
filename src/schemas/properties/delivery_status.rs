use super::*;
/// <https://schema.org/deliveryStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DeliveryStatusProperty {
    #[cfg(any(
        any(feature = "delivery-event-schema", feature = "general-schema-section"),
        doc
    ))]
    DeliveryEvent(DeliveryEvent),
}

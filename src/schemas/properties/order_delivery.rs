use super::*;
/// The delivery of the parcel related to this order or order item.
///
/// https://schema.org/orderDelivery
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OrderDeliveryProperty {
    #[cfg(any(feature = "parcel-delivery-schema", feature = "general-schema-section"))]
    ParcelDelivery(ParcelDelivery),
}

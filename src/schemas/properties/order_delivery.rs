use super::*;
/// <https://schema.org/orderDelivery>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OrderDeliveryProperty {
    #[cfg(any(
        any(feature = "parcel-delivery-schema", feature = "general-schema-section"),
        doc
    ))]
    ParcelDelivery(ParcelDelivery),
}

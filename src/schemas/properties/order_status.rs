use super::*;
/// <https://schema.org/orderStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OrderStatusProperty {
    #[cfg(any(
        any(feature = "order-status-schema", feature = "general-schema-section"),
        doc
    ))]
    OrderStatus(OrderStatus),
}

use super::*;
/// The current status of the order item.
///
/// https://schema.org/orderItemStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OrderItemStatusProperty {
    #[cfg(any(feature = "order-status-schema", feature = "general-schema-section"))]
    OrderStatus(OrderStatus),
}

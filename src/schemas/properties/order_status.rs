use super::*;
/// The current status of the order.
///
/// <https://schema.org/orderStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum OrderStatusProperty {
    #[cfg(any(
        any(feature = "order-status-schema", feature = "general-schema-section"),
        doc
    ))]
    OrderStatus(OrderStatus),
}

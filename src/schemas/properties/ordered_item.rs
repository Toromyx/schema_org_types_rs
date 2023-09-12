use super::*;
/// The item ordered.
///
/// https://schema.org/orderedItem
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OrderedItemProperty {
    #[cfg(any(feature = "order-item-schema", feature = "general-schema-section"))]
    OrderItem(OrderItem),
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
    #[cfg(any(feature = "service-schema", feature = "general-schema-section"))]
    Service(Service),
}

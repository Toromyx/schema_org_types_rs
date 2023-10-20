use super::*;
/// <https://schema.org/orderedItem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OrderedItemProperty {
	#[cfg(any(
		any(feature = "order-item-schema", feature = "general-schema-section"),
		doc
	))]
	OrderItem(OrderItem),
	#[cfg(any(
		any(feature = "product-schema", feature = "general-schema-section"),
		doc
	))]
	Product(Product),
	#[cfg(any(
		any(feature = "service-schema", feature = "general-schema-section"),
		doc
	))]
	Service(Service),
}

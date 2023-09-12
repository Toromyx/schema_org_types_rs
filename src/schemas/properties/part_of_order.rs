use super::*;
/// The overall order the items in this delivery were included in.
///
/// https://schema.org/partOfOrder
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfOrderProperty {
    #[cfg(any(feature = "order-schema", feature = "general-schema-section"))]
    Order(Order),
}

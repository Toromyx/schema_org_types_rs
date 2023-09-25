use super::*;
/// The overall order the items in this delivery were included in.
///
/// <https://schema.org/partOfOrder>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PartOfOrderProperty {
    #[cfg(any(any(feature = "order-schema", feature = "general-schema-section"), doc))]
    Order(Order),
}

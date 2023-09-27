use super::*;
/// <https://schema.org/partOfOrder>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfOrderProperty {
    #[cfg(any(any(feature = "order-schema", feature = "general-schema-section"), doc))]
    Order(Order),
}

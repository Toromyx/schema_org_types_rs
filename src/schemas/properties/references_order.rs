use super::*;
/// The Order(s) related to this Invoice. One or more Orders may be combined into a single Invoice.
///
/// <https://schema.org/referencesOrder>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ReferencesOrderProperty {
    #[cfg(any(any(feature = "order-schema", feature = "general-schema-section"), doc))]
    Order(Order),
}

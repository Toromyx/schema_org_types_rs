use super::*;
/// The Order(s) related to this Invoice. One or more Orders may be combined into a single Invoice.
///
/// https://schema.org/referencesOrder
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReferencesOrderProperty {
    #[cfg(any(feature = "order-schema", feature = "general-schema-section"))]
    Order(Order),
}

use super::*;
/// Date order was placed.
///
/// https://schema.org/orderDate
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OrderDateProperty {
    #[cfg(any(feature = "date-schema", feature = "general-schema-section"))]
    Date(Date),
    #[cfg(any(feature = "date-time-schema", feature = "general-schema-section"))]
    DateTime(DateTime),
}

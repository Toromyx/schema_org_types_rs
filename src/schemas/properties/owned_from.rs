use super::*;
/// The date and time of obtaining the product.
///
/// https://schema.org/ownedFrom
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OwnedFromProperty {
    #[cfg(any(feature = "date-time-schema", feature = "general-schema-section"))]
    DateTime(DateTime),
}
use super::*;
/// <https://schema.org/shippingDestination>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ShippingDestinationProperty {
    #[cfg(any(
        any(feature = "defined-region-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedRegion(DefinedRegion),
}

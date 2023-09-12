use super::*;
/// indicates (possibly multiple) shipping destinations. These can be defined in several ways, e.g. postalCode ranges.
///
/// https://schema.org/shippingDestination
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ShippingDestinationProperty {
    #[cfg(any(feature = "defined-region-schema", feature = "pending-schema-section"))]
    DefinedRegion(DefinedRegion),
}

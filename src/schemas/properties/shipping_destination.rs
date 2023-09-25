use super::*;
/// indicates (possibly multiple) shipping destinations. These can be defined in several ways, e.g. postalCode ranges.
///
/// <https://schema.org/shippingDestination>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ShippingDestinationProperty {
    #[cfg(any(
        any(feature = "defined-region-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedRegion(DefinedRegion),
}

use super::*;
/// Indicates the origin of a shipment, i.e. where it should be coming from.
///
/// https://schema.org/shippingOrigin
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ShippingOriginProperty {
    #[cfg(any(
        any(feature = "defined-region-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedRegion(DefinedRegion),
}

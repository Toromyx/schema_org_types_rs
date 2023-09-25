use super::*;
/// Indicates when shipping to a particular [[shippingDestination]] is not available.
///
/// <https://schema.org/doesNotShip>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DoesNotShipProperty {
    #[cfg(any(
        any(feature = "boolean-schema", feature = "general-schema-section"),
        doc
    ))]
    Boolean(Boolean),
}

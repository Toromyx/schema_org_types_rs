use super::*;
/// Indicates when shipping to a particular [[shippingDestination]] is not available.
///
/// https://schema.org/doesNotShip
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DoesNotShipProperty {
    #[cfg(any(feature = "boolean-schema", feature = "general-schema-section"))]
    Boolean(Boolean),
}

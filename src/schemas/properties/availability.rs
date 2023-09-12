use super::*;
/// The availability of this item&#x2014;for example In stock, Out of stock, Pre-order, etc.
///
/// https://schema.org/availability
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailabilityProperty {
    #[cfg(any(
        feature = "item-availability-schema",
        feature = "general-schema-section"
    ))]
    ItemAvailability(ItemAvailability),
}

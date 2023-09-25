use super::*;
/// The availability of this item&#x2014;for example In stock, Out of stock, Pre-order, etc.
///
/// <https://schema.org/availability>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AvailabilityProperty {
    #[cfg(any(
        any(
            feature = "item-availability-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    ItemAvailability(ItemAvailability),
}

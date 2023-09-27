use super::*;
/// <https://schema.org/availability>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

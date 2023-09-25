use super::*;
/// Indicates some accommodation that this floor plan describes.
///
/// https://schema.org/isPlanForApartment
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IsPlanForApartmentProperty {
    #[cfg(any(
        any(feature = "accommodation-schema", feature = "general-schema-section"),
        doc
    ))]
    Accommodation(Accommodation),
}

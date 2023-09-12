use super::*;
/// Indicates some accommodation that this floor plan describes.
///
/// https://schema.org/isPlanForApartment
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsPlanForApartmentProperty {
    #[cfg(any(feature = "accommodation-schema", feature = "general-schema-section"))]
    Accommodation(Accommodation),
}

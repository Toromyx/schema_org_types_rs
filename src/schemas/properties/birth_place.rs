use super::*;
/// The place where the person was born.
///
/// https://schema.org/birthPlace
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BirthPlaceProperty {
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}

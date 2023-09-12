use super::*;
/// The location where the CreativeWork was created, which may not be the same as the location depicted in the CreativeWork.
///
/// https://schema.org/locationCreated
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LocationCreatedProperty {
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}

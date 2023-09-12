use super::*;
/// A sub property of location. The original location of the object or the agent before the action.
///
/// https://schema.org/fromLocation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FromLocationProperty {
    #[cfg(any(feature = "place-schema", feature = "general-schema-section"))]
    Place(Place),
}

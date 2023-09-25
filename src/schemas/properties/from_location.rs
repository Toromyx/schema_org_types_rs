use super::*;
/// A sub property of location. The original location of the object or the agent before the action.
///
/// <https://schema.org/fromLocation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum FromLocationProperty {
    #[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
    Place(Place),
}

use super::*;
/// An item is an object within the game world that can be collected by a player or, occasionally, a non-player character.
///
/// <https://schema.org/gameItem>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GameItemProperty {
    #[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
    Thing(Thing),
}

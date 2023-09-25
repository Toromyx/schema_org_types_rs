use super::*;
/// A piece of data that represents a particular aspect of a fictional character (skill, power, character points, advantage, disadvantage).
///
/// <https://schema.org/characterAttribute>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CharacterAttributeProperty {
    #[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
    Thing(Thing),
}

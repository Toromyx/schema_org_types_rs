use super::*;
/// A piece of data that represents a particular aspect of a fictional character (skill, power, character points, advantage, disadvantage).
///
/// https://schema.org/characterAttribute
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CharacterAttributeProperty {
    #[cfg(any(feature = "thing-schema", feature = "general-schema-section"))]
    Thing(Thing),
}

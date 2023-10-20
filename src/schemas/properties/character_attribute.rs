use super::*;
/// <https://schema.org/characterAttribute>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CharacterAttributeProperty {
	#[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
	Thing(Thing),
}

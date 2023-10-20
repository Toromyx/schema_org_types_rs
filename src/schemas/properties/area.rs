use super::*;
/// <https://schema.org/area>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AreaProperty {
	#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
	Place(Place),
}

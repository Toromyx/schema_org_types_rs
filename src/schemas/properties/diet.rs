use super::*;
/// <https://schema.org/diet>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DietProperty {
	#[cfg(any(
		any(feature = "diet-schema", feature = "health-lifesci-schema-section"),
		doc
	))]
	Diet(Diet),
}

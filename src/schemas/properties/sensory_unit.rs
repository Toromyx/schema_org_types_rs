use super::*;
/// <https://schema.org/sensoryUnit>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SensoryUnitProperty {
	#[cfg(any(
		any(
			feature = "anatomical-structure-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	AnatomicalStructure(AnatomicalStructure),
	#[cfg(any(
		any(
			feature = "superficial-anatomy-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	SuperficialAnatomy(SuperficialAnatomy),
}

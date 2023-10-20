use super::*;
/// <https://schema.org/regionDrained>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RegionDrainedProperty {
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
			feature = "anatomical-system-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	AnatomicalSystem(AnatomicalSystem),
}

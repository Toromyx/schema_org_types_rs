use super::*;
/// <https://schema.org/legislationApplies>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegislationAppliesProperty {
	#[cfg(any(
		any(feature = "legislation-schema", feature = "pending-schema-section"),
		doc
	))]
	Legislation(Legislation),
}

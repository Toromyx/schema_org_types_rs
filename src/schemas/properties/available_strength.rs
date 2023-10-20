use super::*;
/// <https://schema.org/availableStrength>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableStrengthProperty {
	#[cfg(any(
		any(
			feature = "drug-strength-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	DrugStrength(DrugStrength),
}

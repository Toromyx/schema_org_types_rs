use super::*;
/// <https://schema.org/energyEfficiencyScaleMax>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EnergyEfficiencyScaleMaxProperty {
	#[cfg(any(
		any(
			feature = "eu-energy-efficiency-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	EuEnergyEfficiencyEnumeration(EuEnergyEfficiencyEnumeration),
}

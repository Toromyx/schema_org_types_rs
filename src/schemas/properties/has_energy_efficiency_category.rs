use super::*;
/// Defines the energy efficiency Category (which could be either a rating out of range of values or a yes/no certification) for a product according to an international energy efficiency standard.
///
/// https://schema.org/hasEnergyEfficiencyCategory
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasEnergyEfficiencyCategoryProperty {
    #[cfg(any(
        feature = "energy-efficiency-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    EnergyEfficiencyEnumeration(EnergyEfficiencyEnumeration),
}

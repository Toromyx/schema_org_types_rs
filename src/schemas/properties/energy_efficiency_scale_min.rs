use super::*;
/// Specifies the least energy efficient class on the regulated EU energy consumption scale for the product category a product belongs to. For example, energy consumption for televisions placed on the market after January 1, 2020 is scaled from D to A+++.
///
/// https://schema.org/energyEfficiencyScaleMin
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EnergyEfficiencyScaleMinProperty {
    #[cfg(any(
        feature = "eu-energy-efficiency-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    EuEnergyEfficiencyEnumeration(EuEnergyEfficiencyEnumeration),
}

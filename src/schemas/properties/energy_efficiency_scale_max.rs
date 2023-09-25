use super::*;
/// Specifies the most energy efficient class on the regulated EU energy consumption scale for the product category a product belongs to. For example, energy consumption for televisions placed on the market after January 1, 2020 is scaled from D to A+++.
///
/// <https://schema.org/energyEfficiencyScaleMax>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

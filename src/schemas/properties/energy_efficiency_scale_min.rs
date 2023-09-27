use super::*;
/// <https://schema.org/energyEfficiencyScaleMin>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EnergyEfficiencyScaleMinProperty {
    #[cfg(any(
        any(
            feature = "eu-energy-efficiency-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EuEnergyEfficiencyEnumeration(EuEnergyEfficiencyEnumeration),
}

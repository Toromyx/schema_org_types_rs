use super::*;
/// <https://schema.org/hasEnergyEfficiencyCategory>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasEnergyEfficiencyCategoryProperty {
    #[cfg(any(
        any(
            feature = "energy-efficiency-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EnergyEfficiencyEnumeration(EnergyEfficiencyEnumeration),
}

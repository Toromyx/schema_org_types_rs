use super::*;
/// Defines the energy efficiency Category (which could be either a rating out of range of values or a yes/no certification) for a product according to an international energy efficiency standard.
///
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

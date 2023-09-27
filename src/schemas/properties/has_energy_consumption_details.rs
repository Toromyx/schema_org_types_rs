use super::*;
/// <https://schema.org/hasEnergyConsumptionDetails>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasEnergyConsumptionDetailsProperty {
    #[cfg(any(
        any(
            feature = "energy-consumption-details-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EnergyConsumptionDetails(EnergyConsumptionDetails),
}

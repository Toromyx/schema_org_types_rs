use super::*;
/// Defines the energy efficiency Category (also known as "class" or "rating") for a product according to an international energy efficiency standard.
///
/// https://schema.org/hasEnergyConsumptionDetails
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasEnergyConsumptionDetailsProperty {
    #[cfg(any(
        feature = "energy-consumption-details-schema",
        feature = "pending-schema-section"
    ))]
    EnergyConsumptionDetails(EnergyConsumptionDetails),
}

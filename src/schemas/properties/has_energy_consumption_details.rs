use super::*;
/// Defines the energy efficiency Category (also known as "class" or "rating") for a product according to an international energy efficiency standard.
///
/// <https://schema.org/hasEnergyConsumptionDetails>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

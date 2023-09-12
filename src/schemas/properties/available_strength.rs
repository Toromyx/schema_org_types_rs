use super::*;
/// An available dosage strength for the drug.
///
/// https://schema.org/availableStrength
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableStrengthProperty {
    #[cfg(any(
        feature = "drug-strength-schema",
        feature = "health-lifesci-schema-section"
    ))]
    DrugStrength(DrugStrength),
}

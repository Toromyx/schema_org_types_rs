use super::*;
/// An available dosage strength for the drug.
///
/// <https://schema.org/availableStrength>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AvailableStrengthProperty {
    #[cfg(any(
        any(
            feature = "drug-strength-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    DrugStrength(DrugStrength),
}

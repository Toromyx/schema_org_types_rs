use super::*;
/// <https://schema.org/trialDesign>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TrialDesignProperty {
    #[cfg(any(
        any(
            feature = "medical-trial-design-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalTrialDesign(MedicalTrialDesign),
}

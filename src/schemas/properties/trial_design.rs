use super::*;
/// Specifics about the trial design (enumerated).
///
/// https://schema.org/trialDesign
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TrialDesignProperty {
    #[cfg(any(
        feature = "medical-trial-design-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalTrialDesign(MedicalTrialDesign),
}

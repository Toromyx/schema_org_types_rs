use super::*;
/// One or more alternative conditions considered in the differential diagnosis process as output of a diagnosis process.
///
/// https://schema.org/diagnosis
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DiagnosisProperty {
    #[cfg(any(
        feature = "medical-condition-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalCondition(MedicalCondition),
}

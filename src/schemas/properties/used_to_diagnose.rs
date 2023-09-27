use super::*;
/// <https://schema.org/usedToDiagnose>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum UsedToDiagnoseProperty {
    #[cfg(any(
        any(
            feature = "medical-condition-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalCondition(MedicalCondition),
}

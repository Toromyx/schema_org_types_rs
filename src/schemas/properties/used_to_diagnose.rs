use super::*;
/// A condition the test is used to diagnose.
///
/// https://schema.org/usedToDiagnose
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

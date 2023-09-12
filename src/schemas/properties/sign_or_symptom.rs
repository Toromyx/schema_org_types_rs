use super::*;
/// A sign or symptom of this condition. Signs are objective or physically observable manifestations of the medical condition while symptoms are the subjective experience of the medical condition.
///
/// https://schema.org/signOrSymptom
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SignOrSymptomProperty {
    #[cfg(any(
        feature = "medical-sign-or-symptom-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalSignOrSymptom(MedicalSignOrSymptom),
}

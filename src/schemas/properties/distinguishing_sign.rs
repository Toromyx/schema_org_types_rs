use super::*;
/// One of a set of signs and symptoms that can be used to distinguish this diagnosis from others in the differential diagnosis.
///
/// https://schema.org/distinguishingSign
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DistinguishingSignProperty {
    #[cfg(any(
        feature = "medical-sign-or-symptom-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalSignOrSymptom(MedicalSignOrSymptom),
}

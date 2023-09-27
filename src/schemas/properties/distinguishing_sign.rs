use super::*;
/// <https://schema.org/distinguishingSign>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DistinguishingSignProperty {
    #[cfg(any(
        any(
            feature = "medical-sign-or-symptom-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalSignOrSymptom(MedicalSignOrSymptom),
}

use super::*;
/// <https://schema.org/signOrSymptom>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SignOrSymptomProperty {
    #[cfg(any(
        any(
            feature = "medical-sign-or-symptom-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalSignOrSymptom(MedicalSignOrSymptom),
}

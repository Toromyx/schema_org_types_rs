use super::*;
/// One of a set of signs and symptoms that can be used to distinguish this diagnosis from others in the differential diagnosis.
///
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

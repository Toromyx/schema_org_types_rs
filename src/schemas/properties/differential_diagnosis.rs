use super::*;
/// One of a set of differential diagnoses for the condition. Specifically, a closely-related or competing diagnosis typically considered later in the cognitive process whereby this medical condition is distinguished from others most likely responsible for a similar collection of signs and symptoms to reach the most parsimonious diagnosis or diagnoses in a patient.
///
/// <https://schema.org/differentialDiagnosis>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DifferentialDiagnosisProperty {
    #[cfg(any(
        any(
            feature = "d-dx-element-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    DDxElement(DDxElement),
}

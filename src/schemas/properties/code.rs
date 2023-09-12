use super::*;
/// A medical code for the entity, taken from a controlled vocabulary or ontology such as ICD-9, DiseasesDB, MeSH, SNOMED-CT, RxNorm, etc.
///
/// https://schema.org/code
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CodeProperty {
    #[cfg(any(
        feature = "medical-code-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalCode(MedicalCode),
}

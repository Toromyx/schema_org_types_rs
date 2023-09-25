use super::*;
/// A medical code for the entity, taken from a controlled vocabulary or ontology such as ICD-9, DiseasesDB, MeSH, SNOMED-CT, RxNorm, etc.
///
/// https://schema.org/code
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CodeProperty {
    #[cfg(any(
        any(
            feature = "medical-code-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalCode(MedicalCode),
}

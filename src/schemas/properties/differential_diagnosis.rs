use super::*;
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

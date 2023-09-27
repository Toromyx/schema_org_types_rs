use super::*;
/// <https://schema.org/imagingTechnique>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ImagingTechniqueProperty {
    #[cfg(any(
        any(
            feature = "medical-imaging-technique-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalImagingTechnique(MedicalImagingTechnique),
}

use super::*;
/// Imaging technique used.
///
/// https://schema.org/imagingTechnique
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ImagingTechniqueProperty {
    #[cfg(any(
        feature = "medical-imaging-technique-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalImagingTechnique(MedicalImagingTechnique),
}

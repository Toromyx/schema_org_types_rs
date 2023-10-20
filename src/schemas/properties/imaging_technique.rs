use super::*;
/// <https://schema.org/imagingTechnique>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

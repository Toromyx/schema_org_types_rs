/// <https://schema.org/MedicalImagingTechnique>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalImagingTechnique {
	/// <https://schema.org/CT>
	Ct,
	/// <https://schema.org/MRI>
	Mri,
	/// <https://schema.org/PET>
	Pet,
	/// <https://schema.org/Radiography>
	Radiography,
	/// <https://schema.org/Ultrasound>
	Ultrasound,
	/// <https://schema.org/XRay>
	XRay,
}

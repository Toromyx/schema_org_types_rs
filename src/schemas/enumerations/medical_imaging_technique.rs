/// <https://schema.org/MedicalImagingTechnique>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

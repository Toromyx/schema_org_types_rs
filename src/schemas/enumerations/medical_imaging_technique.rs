/// Any medical imaging modality typically used for diagnostic purposes. Enumerated type.
///
/// https://schema.org/MedicalImagingTechnique
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalImagingTechnique {
    /// X-ray computed tomography imaging.
    ///
    /// https://schema.org/CT
    Ct,
    /// Magnetic resonance imaging.
    ///
    /// https://schema.org/MRI
    Mri,
    /// Positron emission tomography imaging.
    ///
    /// https://schema.org/PET
    Pet,
    /// Radiography is an imaging technique that uses electromagnetic radiation other than visible light, especially X-rays, to view the internal structure of a non-uniformly composed and opaque object such as the human body.
    ///
    /// https://schema.org/Radiography
    Radiography,
    /// Ultrasound imaging.
    ///
    /// https://schema.org/Ultrasound
    Ultrasound,
    /// X-ray imaging.
    ///
    /// https://schema.org/XRay
    XRay,
}

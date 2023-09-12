/// A type of physical examination of a patient performed by a physician.
///
/// https://schema.org/PhysicalExam
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhysicalExam {
    /// Abdomen clinical examination.
    ///
    /// https://schema.org/Abdomen
    Abdomen,
    /// Appearance assessment with clinical examination.
    ///
    /// https://schema.org/Appearance
    Appearance,
    /// Cardiovascular system assessment with clinical examination.
    ///
    /// https://schema.org/CardiovascularExam
    CardiovascularExam,
    /// Ear function assessment with clinical examination.
    ///
    /// https://schema.org/Ear
    Ear,
    /// Eye or ophthalmological function assessment with clinical examination.
    ///
    /// https://schema.org/Eye
    Eye,
    /// Genitourinary system function assessment with clinical examination.
    ///
    /// https://schema.org/Genitourinary
    Genitourinary,
    /// Head assessment with clinical examination.
    ///
    /// https://schema.org/Head
    Head,
    /// Lung and respiratory system clinical examination.
    ///
    /// https://schema.org/Lung
    Lung,
    /// Musculoskeletal system clinical examination.
    ///
    /// https://schema.org/MusculoskeletalExam
    MusculoskeletalExam,
    /// Neck assessment with clinical examination.
    ///
    /// https://schema.org/Neck
    Neck,
    /// Neurological system clinical examination.
    ///
    /// https://schema.org/Neuro
    Neuro,
    /// Nose function assessment with clinical examination.
    ///
    /// https://schema.org/Nose
    Nose,
    /// Skin assessment with clinical examination.
    ///
    /// https://schema.org/Skin
    Skin,
    /// Throat assessment with  clinical examination.
    ///
    /// https://schema.org/Throat
    Throat,
}

/// <https://schema.org/PhysicalExam>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhysicalExam {
    /// <https://schema.org/Abdomen>
    Abdomen,
    /// <https://schema.org/Appearance>
    Appearance,
    /// <https://schema.org/CardiovascularExam>
    CardiovascularExam,
    /// <https://schema.org/Ear>
    Ear,
    /// <https://schema.org/Eye>
    Eye,
    /// <https://schema.org/Genitourinary>
    Genitourinary,
    /// <https://schema.org/Head>
    Head,
    /// <https://schema.org/Lung>
    Lung,
    /// <https://schema.org/MusculoskeletalExam>
    MusculoskeletalExam,
    /// <https://schema.org/Neck>
    Neck,
    /// <https://schema.org/Neuro>
    Neuro,
    /// <https://schema.org/Nose>
    Nose,
    /// <https://schema.org/Skin>
    Skin,
    /// <https://schema.org/Throat>
    Throat,
}

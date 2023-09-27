/// <https://schema.org/PhysicalExam>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

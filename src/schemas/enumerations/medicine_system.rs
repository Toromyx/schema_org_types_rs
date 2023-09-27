/// <https://schema.org/MedicineSystem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicineSystem {
    /// <https://schema.org/Ayurvedic>
    Ayurvedic,
    /// <https://schema.org/Chiropractic>
    Chiropractic,
    /// <https://schema.org/Homeopathic>
    Homeopathic,
    /// <https://schema.org/Osteopathic>
    Osteopathic,
    /// <https://schema.org/TraditionalChinese>
    TraditionalChinese,
    /// <https://schema.org/WesternConventional>
    WesternConventional,
}

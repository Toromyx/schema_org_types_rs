/// <https://schema.org/MedicineSystem>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

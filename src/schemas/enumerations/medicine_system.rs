/// Systems of medical practice.
///
/// <https://schema.org/MedicineSystem>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicineSystem {
    /// A system of medicine that originated in India over thousands of years and that focuses on integrating and balancing the body, mind, and spirit.
    ///
    /// <https://schema.org/Ayurvedic>
    Ayurvedic,
    /// A system of medicine focused on the relationship between the body's structure, mainly the spine, and its functioning.
    ///
    /// <https://schema.org/Chiropractic>
    Chiropractic,
    /// A system of medicine based on the principle that a disease can be cured by a substance that produces similar symptoms in healthy people.
    ///
    /// <https://schema.org/Homeopathic>
    Homeopathic,
    /// A system of medicine focused on promoting the body's innate ability to heal itself.
    ///
    /// <https://schema.org/Osteopathic>
    Osteopathic,
    /// A system of medicine based on common theoretical concepts that originated in China and evolved over thousands of years, that uses herbs, acupuncture, exercise, massage, dietary therapy, and other methods to treat a wide range of conditions.
    ///
    /// <https://schema.org/TraditionalChinese>
    TraditionalChinese,
    /// The conventional Western system of medicine, that aims to apply the best available evidence gained from the scientific method to clinical decision making. Also known as conventional or Western medicine.
    ///
    /// <https://schema.org/WesternConventional>
    WesternConventional,
}

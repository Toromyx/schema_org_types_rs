/// <https://schema.org/MedicalSpecialty>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalSpecialty {
    /// <https://schema.org/Anesthesia>
    Anesthesia,
    /// <https://schema.org/Cardiovascular>
    Cardiovascular,
    /// <https://schema.org/CommunityHealth>
    CommunityHealth,
    /// <https://schema.org/Dentistry>
    Dentistry,
    /// <https://schema.org/Dermatologic>
    Dermatologic,
    /// <https://schema.org/Dermatology>
    Dermatology,
    /// <https://schema.org/DietNutrition>
    DietNutrition,
    /// <https://schema.org/Emergency>
    Emergency,
    /// <https://schema.org/Endocrine>
    Endocrine,
    /// <https://schema.org/Gastroenterologic>
    Gastroenterologic,
    /// <https://schema.org/Genetic>
    Genetic,
    /// <https://schema.org/Geriatric>
    Geriatric,
    /// <https://schema.org/Gynecologic>
    Gynecologic,
    /// <https://schema.org/Hematologic>
    Hematologic,
    /// <https://schema.org/Infectious>
    Infectious,
    /// <https://schema.org/LaboratoryScience>
    LaboratoryScience,
    /// <https://schema.org/Midwifery>
    Midwifery,
    /// <https://schema.org/Musculoskeletal>
    Musculoskeletal,
    /// <https://schema.org/Neurologic>
    Neurologic,
    /// <https://schema.org/Nursing>
    Nursing,
    /// <https://schema.org/Obstetric>
    Obstetric,
    /// <https://schema.org/Oncologic>
    Oncologic,
    /// <https://schema.org/Optometric>
    Optometric,
    /// <https://schema.org/Otolaryngologic>
    Otolaryngologic,
    /// <https://schema.org/Pathology>
    Pathology,
    /// <https://schema.org/Pediatric>
    Pediatric,
    /// <https://schema.org/PharmacySpecialty>
    PharmacySpecialty,
    /// <https://schema.org/Physiotherapy>
    Physiotherapy,
    /// <https://schema.org/PlasticSurgery>
    PlasticSurgery,
    /// <https://schema.org/Podiatric>
    Podiatric,
    /// <https://schema.org/PrimaryCare>
    PrimaryCare,
    /// <https://schema.org/Psychiatric>
    Psychiatric,
    /// <https://schema.org/PublicHealth>
    PublicHealth,
    /// <https://schema.org/Pulmonary>
    Pulmonary,
    /// <https://schema.org/Radiography>
    Radiography,
    /// <https://schema.org/Renal>
    Renal,
    /// <https://schema.org/RespiratoryTherapy>
    RespiratoryTherapy,
    /// <https://schema.org/Rheumatologic>
    Rheumatologic,
    /// <https://schema.org/SpeechPathology>
    SpeechPathology,
    /// <https://schema.org/Surgical>
    Surgical,
    /// <https://schema.org/Toxicologic>
    Toxicologic,
    /// <https://schema.org/Urologic>
    Urologic,
}

/// Any specific branch of medical science or practice. Medical specialities include clinical specialties that pertain to particular organ systems and their respective disease states, as well as allied health specialties. Enumerated type.
///
/// <https://schema.org/MedicalSpecialty>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalSpecialty {
    /// A specific branch of medical science that pertains to study of anesthetics and their application.
    ///
    /// <https://schema.org/Anesthesia>
    Anesthesia,
    /// A specific branch of medical science that pertains to diagnosis and treatment of disorders of heart and vasculature.
    ///
    /// <https://schema.org/Cardiovascular>
    Cardiovascular,
    /// A field of public health focusing on improving health characteristics of a defined population in relation with their geographical or environment areas.
    ///
    /// <https://schema.org/CommunityHealth>
    CommunityHealth,
    /// A branch of medicine that is involved in the dental care.
    ///
    /// <https://schema.org/Dentistry>
    Dentistry,
    /// Something relating to or practicing dermatology.
    ///
    /// <https://schema.org/Dermatologic>
    Dermatologic,
    /// A specific branch of medical science that pertains to diagnosis and treatment of disorders of skin.
    ///
    /// <https://schema.org/Dermatology>
    Dermatology,
    /// Dietetics and nutrition as a medical specialty.
    ///
    /// <https://schema.org/DietNutrition>
    DietNutrition,
    /// A specific branch of medical science that deals with the evaluation and initial treatment of medical conditions caused by trauma or sudden illness.
    ///
    /// <https://schema.org/Emergency>
    Emergency,
    /// A specific branch of medical science that pertains to diagnosis and treatment of disorders of endocrine glands and their secretions.
    ///
    /// <https://schema.org/Endocrine>
    Endocrine,
    /// A specific branch of medical science that pertains to diagnosis and treatment of disorders of digestive system.
    ///
    /// <https://schema.org/Gastroenterologic>
    Gastroenterologic,
    /// A specific branch of medical science that pertains to hereditary transmission and the variation of inherited characteristics and disorders.
    ///
    /// <https://schema.org/Genetic>
    Genetic,
    /// A specific branch of medical science that is concerned with the diagnosis and treatment of diseases, debilities and provision of care to the aged.
    ///
    /// <https://schema.org/Geriatric>
    Geriatric,
    /// A specific branch of medical science that pertains to the health care of women, particularly in the diagnosis and treatment of disorders affecting the female reproductive system.
    ///
    /// <https://schema.org/Gynecologic>
    Gynecologic,
    /// A specific branch of medical science that pertains to diagnosis and treatment of disorders of blood and blood producing organs.
    ///
    /// <https://schema.org/Hematologic>
    Hematologic,
    /// Something in medical science that pertains to infectious diseases, i.e. caused by bacterial, viral, fungal or parasitic infections.
    ///
    /// <https://schema.org/Infectious>
    Infectious,
    /// A medical science pertaining to chemical, hematological, immunologic, microscopic, or bacteriological diagnostic analyses or research.
    ///
    /// <https://schema.org/LaboratoryScience>
    LaboratoryScience,
    /// A nurse-like health profession that deals with pregnancy, childbirth, and the postpartum period (including care of the newborn), besides sexual and reproductive health of women throughout their lives.
    ///
    /// <https://schema.org/Midwifery>
    Midwifery,
    /// A specific branch of medical science that pertains to diagnosis and treatment of disorders of muscles, ligaments and skeletal system.
    ///
    /// <https://schema.org/Musculoskeletal>
    Musculoskeletal,
    /// A specific branch of medical science that studies the nerves and nervous system and its respective disease states.
    ///
    /// <https://schema.org/Neurologic>
    Neurologic,
    /// A health profession of a person formally educated and trained in the care of the sick or infirm person.
    ///
    /// <https://schema.org/Nursing>
    Nursing,
    /// A specific branch of medical science that specializes in the care of women during the prenatal and postnatal care and with the delivery of the child.
    ///
    /// <https://schema.org/Obstetric>
    Obstetric,
    /// A specific branch of medical science that deals with benign and malignant tumors, including the study of their development, diagnosis, treatment and prevention.
    ///
    /// <https://schema.org/Oncologic>
    Oncologic,
    /// The science or practice of testing visual acuity and prescribing corrective lenses.
    ///
    /// <https://schema.org/Optometric>
    Optometric,
    /// A specific branch of medical science that is concerned with the ear, nose and throat and their respective disease states.
    ///
    /// <https://schema.org/Otolaryngologic>
    Otolaryngologic,
    /// A specific branch of medical science that is concerned with the study of the cause, origin and nature of a disease state, including its consequences as a result of manifestation of the disease. In clinical care, the term is used to designate a branch of medicine using laboratory tests to diagnose and determine the prognostic significance of illness.
    ///
    /// <https://schema.org/Pathology>
    Pathology,
    /// A specific branch of medical science that specializes in the care of infants, children and adolescents.
    ///
    /// <https://schema.org/Pediatric>
    Pediatric,
    /// The practice or art and science of preparing and dispensing drugs and medicines.
    ///
    /// <https://schema.org/PharmacySpecialty>
    PharmacySpecialty,
    /// The practice of treatment of disease, injury, or deformity by physical methods such as massage, heat treatment, and exercise rather than by drugs or surgery.
    ///
    /// <https://schema.org/Physiotherapy>
    Physiotherapy,
    /// A specific branch of medical science that pertains to therapeutic or cosmetic repair or re-formation of missing, injured or malformed tissues or body parts by manual and instrumental means.
    ///
    /// <https://schema.org/PlasticSurgery>
    PlasticSurgery,
    /// Podiatry is the care of the human foot, especially the diagnosis and treatment of foot disorders.
    ///
    /// <https://schema.org/Podiatric>
    Podiatric,
    /// The medical care by a physician, or other health-care professional, who is the patient's first contact with the health-care system and who may recommend a specialist if necessary.
    ///
    /// <https://schema.org/PrimaryCare>
    PrimaryCare,
    /// A specific branch of medical science that is concerned with the study, treatment, and prevention of mental illness, using both medical and psychological therapies.
    ///
    /// <https://schema.org/Psychiatric>
    Psychiatric,
    /// Branch of medicine that pertains to the health services to improve and protect community health, especially epidemiology, sanitation, immunization, and preventive medicine.
    ///
    /// <https://schema.org/PublicHealth>
    PublicHealth,
    /// A specific branch of medical science that pertains to the study of the respiratory system and its respective disease states.
    ///
    /// <https://schema.org/Pulmonary>
    Pulmonary,
    /// Radiography is an imaging technique that uses electromagnetic radiation other than visible light, especially X-rays, to view the internal structure of a non-uniformly composed and opaque object such as the human body.
    ///
    /// <https://schema.org/Radiography>
    Radiography,
    /// A specific branch of medical science that pertains to the study of the kidneys and its respective disease states.
    ///
    /// <https://schema.org/Renal>
    Renal,
    /// The therapy that is concerned with the maintenance or improvement of respiratory function (as in patients with pulmonary disease).
    ///
    /// <https://schema.org/RespiratoryTherapy>
    RespiratoryTherapy,
    /// A specific branch of medical science that deals with the study and treatment of rheumatic, autoimmune or joint diseases.
    ///
    /// <https://schema.org/Rheumatologic>
    Rheumatologic,
    /// The scientific study and treatment of defects, disorders, and malfunctions of speech and voice, as stuttering, lisping, or lalling, and of language disturbances, as aphasia or delayed language acquisition.
    ///
    /// <https://schema.org/SpeechPathology>
    SpeechPathology,
    /// A specific branch of medical science that pertains to treating diseases, injuries and deformities by manual and instrumental means.
    ///
    /// <https://schema.org/Surgical>
    Surgical,
    /// A specific branch of medical science that is concerned with poisons, their nature, effects and detection and involved in the treatment of poisoning.
    ///
    /// <https://schema.org/Toxicologic>
    Toxicologic,
    /// A specific branch of medical science that is concerned with the diagnosis and treatment of diseases pertaining to the urinary tract and the urogenital system.
    ///
    /// <https://schema.org/Urologic>
    Urologic,
}

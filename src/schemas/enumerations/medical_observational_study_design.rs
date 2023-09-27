/// <https://schema.org/MedicalObservationalStudyDesign>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalObservationalStudyDesign {
    /// <https://schema.org/CaseSeries>
    CaseSeries,
    /// <https://schema.org/CohortStudy>
    CohortStudy,
    /// <https://schema.org/CrossSectional>
    CrossSectional,
    /// <https://schema.org/Longitudinal>
    Longitudinal,
    /// <https://schema.org/Observational>
    Observational,
    /// <https://schema.org/Registry>
    Registry,
}

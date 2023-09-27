/// <https://schema.org/MedicalObservationalStudyDesign>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

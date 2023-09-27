/// <https://schema.org/MedicalStudyStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalStudyStatus {
    /// <https://schema.org/ActiveNotRecruiting>
    ActiveNotRecruiting,
    /// <https://schema.org/Completed>
    Completed,
    /// <https://schema.org/EnrollingByInvitation>
    EnrollingByInvitation,
    /// <https://schema.org/NotYetRecruiting>
    NotYetRecruiting,
    /// <https://schema.org/Recruiting>
    Recruiting,
    /// <https://schema.org/ResultsAvailable>
    ResultsAvailable,
    /// <https://schema.org/ResultsNotAvailable>
    ResultsNotAvailable,
    /// <https://schema.org/Suspended>
    Suspended,
    /// <https://schema.org/Terminated>
    Terminated,
    /// <https://schema.org/Withdrawn>
    Withdrawn,
}

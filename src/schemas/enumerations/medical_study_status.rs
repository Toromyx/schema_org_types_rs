/// The status of a medical study. Enumerated type.
///
/// https://schema.org/MedicalStudyStatus
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MedicalStudyStatus {
    /// Active, but not recruiting new participants.
    ///
    /// https://schema.org/ActiveNotRecruiting
    ActiveNotRecruiting,
    /// Completed.
    ///
    /// https://schema.org/Completed
    Completed,
    /// Enrolling participants by invitation only.
    ///
    /// https://schema.org/EnrollingByInvitation
    EnrollingByInvitation,
    /// Not yet recruiting.
    ///
    /// https://schema.org/NotYetRecruiting
    NotYetRecruiting,
    /// Recruiting participants.
    ///
    /// https://schema.org/Recruiting
    Recruiting,
    /// Results are available.
    ///
    /// https://schema.org/ResultsAvailable
    ResultsAvailable,
    /// Results are not available.
    ///
    /// https://schema.org/ResultsNotAvailable
    ResultsNotAvailable,
    /// Suspended.
    ///
    /// https://schema.org/Suspended
    Suspended,
    /// Terminated.
    ///
    /// https://schema.org/Terminated
    Terminated,
    /// Withdrawn.
    ///
    /// https://schema.org/Withdrawn
    Withdrawn,
}

/// <https://schema.org/ActionStatusType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionStatusType {
    /// <https://schema.org/ActiveActionStatus>
    ActiveActionStatus,
    /// <https://schema.org/CompletedActionStatus>
    CompletedActionStatus,
    /// <https://schema.org/FailedActionStatus>
    FailedActionStatus,
    /// <https://schema.org/PotentialActionStatus>
    PotentialActionStatus,
}

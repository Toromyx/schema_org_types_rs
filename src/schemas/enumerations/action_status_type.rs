/// <https://schema.org/ActionStatusType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

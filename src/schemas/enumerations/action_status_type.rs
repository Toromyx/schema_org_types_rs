/// The status of an Action.
///
/// <https://schema.org/ActionStatusType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ActionStatusType {
    /// An in-progress action (e.g., while watching the movie, or driving to a location).
    ///
    /// <https://schema.org/ActiveActionStatus>
    ActiveActionStatus,
    /// An action that has already taken place.
    ///
    /// <https://schema.org/CompletedActionStatus>
    CompletedActionStatus,
    /// An action that failed to complete. The action's error property and the HTTP return code contain more information about the failure.
    ///
    /// <https://schema.org/FailedActionStatus>
    FailedActionStatus,
    /// A description of an action that is supported.
    ///
    /// <https://schema.org/PotentialActionStatus>
    PotentialActionStatus,
}

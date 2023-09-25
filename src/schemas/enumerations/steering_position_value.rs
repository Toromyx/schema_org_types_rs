/// A value indicating a steering position.
///
/// <https://schema.org/SteeringPositionValue>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum SteeringPositionValue {
    /// The steering position is on the left side of the vehicle (viewed from the main direction of driving).
    ///
    /// <https://schema.org/LeftHandDriving>
    LeftHandDriving,
    /// The steering position is on the right side of the vehicle (viewed from the main direction of driving).
    ///
    /// <https://schema.org/RightHandDriving>
    RightHandDriving,
}

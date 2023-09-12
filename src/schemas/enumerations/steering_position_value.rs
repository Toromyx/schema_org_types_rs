/// A value indicating a steering position.
///
/// https://schema.org/SteeringPositionValue
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SteeringPositionValue {
    /// The steering position is on the left side of the vehicle (viewed from the main direction of driving).
    ///
    /// https://schema.org/LeftHandDriving
    LeftHandDriving,
    /// The steering position is on the right side of the vehicle (viewed from the main direction of driving).
    ///
    /// https://schema.org/RightHandDriving
    RightHandDriving,
}

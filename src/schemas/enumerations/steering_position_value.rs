/// <https://schema.org/SteeringPositionValue>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum SteeringPositionValue {
    /// <https://schema.org/LeftHandDriving>
    LeftHandDriving,
    /// <https://schema.org/RightHandDriving>
    RightHandDriving,
}

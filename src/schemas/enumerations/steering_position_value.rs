/// <https://schema.org/SteeringPositionValue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SteeringPositionValue {
	/// <https://schema.org/LeftHandDriving>
	LeftHandDriving,
	/// <https://schema.org/RightHandDriving>
	RightHandDriving,
}

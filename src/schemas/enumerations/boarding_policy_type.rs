/// <https://schema.org/BoardingPolicyType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoardingPolicyType {
	/// <https://schema.org/GroupBoardingPolicy>
	GroupBoardingPolicy,
	/// <https://schema.org/ZoneBoardingPolicy>
	ZoneBoardingPolicy,
}

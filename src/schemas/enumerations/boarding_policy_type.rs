/// <https://schema.org/BoardingPolicyType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum BoardingPolicyType {
    /// <https://schema.org/GroupBoardingPolicy>
    GroupBoardingPolicy,
    /// <https://schema.org/ZoneBoardingPolicy>
    ZoneBoardingPolicy,
}

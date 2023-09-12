/// A type of boarding policy used by an airline.
///
/// https://schema.org/BoardingPolicyType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoardingPolicyType {
    /// The airline boards by groups based on check-in time, priority, etc.
    ///
    /// https://schema.org/GroupBoardingPolicy
    GroupBoardingPolicy,
    /// The airline boards by zones of the plane.
    ///
    /// https://schema.org/ZoneBoardingPolicy
    ZoneBoardingPolicy,
}

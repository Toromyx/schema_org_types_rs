/// A type of boarding policy used by an airline.
///
/// https://schema.org/BoardingPolicyType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

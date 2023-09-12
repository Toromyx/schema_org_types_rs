/// A list of possible statuses for the legal force of a legislation.
///
/// https://schema.org/LegalForceStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LegalForceStatus {
    /// Indicates that a legislation is in force.
    ///
    /// https://schema.org/InForce
    InForce,
    /// Indicates that a legislation is currently not in force.
    ///
    /// https://schema.org/NotInForce
    NotInForce,
    /// Indicates that parts of the legislation are in force, and parts are not.
    ///
    /// https://schema.org/PartiallyInForce
    PartiallyInForce,
}

use super::*;
/// Any membership in a frequent flyer, hotel loyalty program, etc. being applied to the reservation.
///
/// https://schema.org/programMembershipUsed
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProgramMembershipUsedProperty {
    #[cfg(any(
        feature = "program-membership-schema",
        feature = "general-schema-section"
    ))]
    ProgramMembership(ProgramMembership),
}

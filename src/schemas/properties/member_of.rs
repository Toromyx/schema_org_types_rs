use super::*;
/// An Organization (or ProgramMembership) to which this Person or Organization belongs.
///
/// https://schema.org/memberOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MemberOfProperty {
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
    #[cfg(any(
        feature = "program-membership-schema",
        feature = "general-schema-section"
    ))]
    ProgramMembership(ProgramMembership),
}

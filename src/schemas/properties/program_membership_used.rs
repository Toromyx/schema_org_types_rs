use super::*;
/// <https://schema.org/programMembershipUsed>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ProgramMembershipUsedProperty {
    #[cfg(any(
        any(
            feature = "program-membership-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    ProgramMembership(ProgramMembership),
}

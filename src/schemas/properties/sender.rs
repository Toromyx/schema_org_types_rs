use super::*;
/// A sub property of participant. The participant who is at the sending end of the action.
///
/// https://schema.org/sender
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SenderProperty {
    #[cfg(any(feature = "audience-schema", feature = "general-schema-section"))]
    Audience(Audience),
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}

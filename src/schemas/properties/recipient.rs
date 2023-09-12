use super::*;
/// A sub property of participant. The participant who is at the receiving end of the action.
///
/// https://schema.org/recipient
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RecipientProperty {
    #[cfg(any(feature = "audience-schema", feature = "general-schema-section"))]
    Audience(Audience),
    #[cfg(any(feature = "contact-point-schema", feature = "general-schema-section"))]
    ContactPoint(ContactPoint),
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}

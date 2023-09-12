use super::*;
/// A sub property of recipient. The recipient copied on a message.
///
/// https://schema.org/ccRecipient
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CcRecipientProperty {
    #[cfg(any(feature = "contact-point-schema", feature = "general-schema-section"))]
    ContactPoint(ContactPoint),
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}

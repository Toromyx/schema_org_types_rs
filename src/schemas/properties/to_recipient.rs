use super::*;
/// A sub property of recipient. The recipient who was directly sent the message.
///
/// <https://schema.org/toRecipient>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ToRecipientProperty {
    #[cfg(any(
        any(feature = "audience-schema", feature = "general-schema-section"),
        doc
    ))]
    Audience(Audience),
    #[cfg(any(
        any(feature = "contact-point-schema", feature = "general-schema-section"),
        doc
    ))]
    ContactPoint(ContactPoint),
    #[cfg(any(
        any(feature = "organization-schema", feature = "general-schema-section"),
        doc
    ))]
    Organization(Organization),
    #[cfg(any(
        any(feature = "person-schema", feature = "general-schema-section"),
        doc
    ))]
    Person(Person),
}

use super::*;
/// The person, organization, contact point, or audience that has been granted this permission.
///
/// https://schema.org/grantee
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GranteeProperty {
    #[cfg(any(feature = "audience-schema", feature = "general-schema-section"))]
    Audience(Audience),
    #[cfg(any(feature = "contact-point-schema", feature = "general-schema-section"))]
    ContactPoint(ContactPoint),
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}

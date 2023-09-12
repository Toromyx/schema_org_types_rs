use super::*;
/// The person or organization the reservation or ticket is for.
///
/// https://schema.org/underName
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum UnderNameProperty {
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}

use super::*;
/// An individual or organization that has some kind of responsibility for the legislation. Typically the ministry who is/was in charge of elaborating the legislation, or the adressee for potential questions about the legislation once it is published.
///
/// <https://schema.org/legislationResponsible>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LegislationResponsibleProperty {
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

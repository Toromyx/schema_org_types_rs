use super::*;
/// An organization that the person is an alumni of.
///
/// <https://schema.org/alumniOf>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AlumniOfProperty {
    #[cfg(any(
        any(
            feature = "educational-organization-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    EducationalOrganization(EducationalOrganization),
    #[cfg(any(
        any(feature = "organization-schema", feature = "general-schema-section"),
        doc
    ))]
    Organization(Organization),
}

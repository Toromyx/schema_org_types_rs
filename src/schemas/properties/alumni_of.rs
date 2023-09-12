use super::*;
/// An organization that the person is an alumni of.
///
/// https://schema.org/alumniOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AlumniOfProperty {
    #[cfg(any(
        feature = "educational-organization-schema",
        feature = "general-schema-section"
    ))]
    EducationalOrganization(EducationalOrganization),
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
}

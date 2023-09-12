use super::*;
/// A relationship between two organizations where the first includes the second, e.g., as a subsidiary. See also: the more specific 'department' property.
///
/// https://schema.org/subOrganization
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SubOrganizationProperty {
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
}

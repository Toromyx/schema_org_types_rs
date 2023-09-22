use super::*;
/// Organizations that the person works for.
///
/// https://schema.org/worksFor
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum WorksForProperty {
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
}
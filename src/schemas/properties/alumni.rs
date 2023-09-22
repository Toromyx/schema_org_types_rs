use super::*;
/// Alumni of an organization.
///
/// https://schema.org/alumni
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AlumniProperty {
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
}
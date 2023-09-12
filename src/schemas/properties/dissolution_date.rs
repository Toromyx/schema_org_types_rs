use super::*;
/// The date that this organization was dissolved.
///
/// https://schema.org/dissolutionDate
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DissolutionDateProperty {
    #[cfg(any(feature = "date-schema", feature = "general-schema-section"))]
    Date(Date),
}

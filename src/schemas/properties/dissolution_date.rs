use super::*;
/// The date that this organization was dissolved.
///
/// <https://schema.org/dissolutionDate>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DissolutionDateProperty {
    #[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
    Date(Date),
}

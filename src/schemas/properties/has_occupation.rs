use super::*;
/// The Person's occupation. For past professions, use Role for expressing dates.
///
/// <https://schema.org/hasOccupation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasOccupationProperty {
    #[cfg(any(
        any(feature = "occupation-schema", feature = "general-schema-section"),
        doc
    ))]
    Occupation(Occupation),
}

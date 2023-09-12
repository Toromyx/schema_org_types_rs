use super::*;
/// The Person's occupation. For past professions, use Role for expressing dates.
///
/// https://schema.org/hasOccupation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasOccupationProperty {
    #[cfg(any(feature = "occupation-schema", feature = "general-schema-section"))]
    Occupation(Occupation),
}

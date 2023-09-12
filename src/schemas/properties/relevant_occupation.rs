use super::*;
/// The Occupation for the JobPosting.
///
/// https://schema.org/relevantOccupation
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RelevantOccupationProperty {
    #[cfg(any(feature = "occupation-schema", feature = "general-schema-section"))]
    Occupation(Occupation),
}

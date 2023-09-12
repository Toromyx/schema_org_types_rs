use super::*;
/// The number of positions open for this job posting. Use a positive integer. Do not use if the number of positions is unclear or not known.
///
/// https://schema.org/totalJobOpenings
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TotalJobOpeningsProperty {
    #[cfg(any(feature = "integer-schema", feature = "general-schema-section"))]
    Integer(Integer),
}

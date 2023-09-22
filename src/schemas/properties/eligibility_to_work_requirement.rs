use super::*;
/// The legal requirements such as citizenship, visa and other documentation required for an applicant to this job.
///
/// https://schema.org/eligibilityToWorkRequirement
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EligibilityToWorkRequirementProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
use super::*;
/// The date on which a successful applicant for this job would be expected to start work. Choose a specific date in the future or use the jobImmediateStart property to indicate the position is to be filled as soon as possible.
///
/// https://schema.org/jobStartDate
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum JobStartDateProperty {
    #[cfg(any(feature = "date-schema", feature = "general-schema-section"))]
    Date(Date),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}

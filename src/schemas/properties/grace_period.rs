use super::*;
/// The period of time after any due date that the borrower has to fulfil its obligations before a default (failure to pay) is deemed to have occurred.
///
/// https://schema.org/gracePeriod
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GracePeriodProperty {
    #[cfg(any(feature = "duration-schema", feature = "general-schema-section"))]
    Duration(Duration),
}

use super::*;
/// The date/time at which the message has been read by the recipient if a single recipient exists.
///
/// https://schema.org/dateRead
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DateReadProperty {
    #[cfg(any(feature = "date-schema", feature = "general-schema-section"))]
    Date(Date),
    #[cfg(any(feature = "date-time-schema", feature = "general-schema-section"))]
    DateTime(DateTime),
}

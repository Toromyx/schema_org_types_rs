use super::*;
/// Defines a [[Date]] or [[DateTime]] during which a scheduled [[Event]] will not take place. The property allows exceptions to
/// a [[Schedule]] to be specified. If an exception is specified as a [[DateTime]] then only the event that would have started at that specific date and time
/// should be excluded from the schedule. If an exception is specified as a [[Date]] then any event that is scheduled for that 24 hour period should be
/// excluded from the schedule. This allows a whole day to be excluded from the schedule without having to itemise every scheduled event.
///
/// <https://schema.org/exceptDate>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ExceptDateProperty {
    #[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
    Date(Date),
    #[cfg(any(
        any(feature = "date-time-schema", feature = "general-schema-section"),
        doc
    ))]
    DateTime(DateTime),
}

use super::*;
/// The date/time the message was received if a single recipient exists.
///
/// <https://schema.org/dateReceived>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DateReceivedProperty {
    #[cfg(any(
        any(feature = "date-time-schema", feature = "general-schema-section"),
        doc
    ))]
    DateTime(DateTime),
}

use super::*;
/// The time when a passenger can check into the flight online.
///
/// <https://schema.org/webCheckinTime>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum WebCheckinTimeProperty {
    #[cfg(any(
        any(feature = "date-time-schema", feature = "general-schema-section"),
        doc
    ))]
    DateTime(DateTime),
}

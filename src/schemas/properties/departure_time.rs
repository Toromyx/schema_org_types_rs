use super::*;
/// The expected departure time.
///
/// <https://schema.org/departureTime>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DepartureTimeProperty {
    #[cfg(any(
        any(feature = "date-time-schema", feature = "general-schema-section"),
        doc
    ))]
    DateTime(DateTime),
    #[cfg(any(any(feature = "time-schema", feature = "general-schema-section"), doc))]
    Time(Time),
}

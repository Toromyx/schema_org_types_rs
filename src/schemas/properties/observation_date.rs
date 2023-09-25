use super::*;
/// The observationDate of an [[Observation]].
///
/// <https://schema.org/observationDate>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ObservationDateProperty {
    #[cfg(any(
        any(feature = "date-time-schema", feature = "general-schema-section"),
        doc
    ))]
    DateTime(DateTime),
}

use super::*;
/// The expected length of time to complete the program if attending full-time.
///
/// <https://schema.org/timeToComplete>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TimeToCompleteProperty {
    #[cfg(any(
        any(feature = "duration-schema", feature = "general-schema-section"),
        doc
    ))]
    Duration(Duration),
}

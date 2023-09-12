use super::*;
/// The estimated time the flight will take.
///
/// https://schema.org/estimatedFlightDuration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EstimatedFlightDurationProperty {
    #[cfg(any(feature = "duration-schema", feature = "general-schema-section"))]
    Duration(Duration),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}

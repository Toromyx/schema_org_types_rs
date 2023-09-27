use super::*;
/// <https://schema.org/cutoffTime>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CutoffTimeProperty {
    #[cfg(any(any(feature = "time-schema", feature = "general-schema-section"), doc))]
    Time(Time),
}

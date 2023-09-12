use super::*;
/// The opening hour of the place or service on the given day(s) of the week.
///
/// https://schema.org/opens
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OpensProperty {
    #[cfg(any(feature = "time-schema", feature = "general-schema-section"))]
    Time(Time),
}

use super::*;
/// The closing hour of the place or service on the given day(s) of the week.
///
/// https://schema.org/closes
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ClosesProperty {
    #[cfg(any(feature = "time-schema", feature = "general-schema-section"))]
    Time(Time),
}

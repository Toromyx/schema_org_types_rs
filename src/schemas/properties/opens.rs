use super::*;
/// The opening hour of the place or service on the given day(s) of the week.
///
/// https://schema.org/opens
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum OpensProperty {
    #[cfg(any(any(feature = "time-schema", feature = "general-schema-section"), doc))]
    Time(Time),
}

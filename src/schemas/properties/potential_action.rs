use super::*;
/// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
///
/// https://schema.org/potentialAction
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PotentialActionProperty {
    #[cfg(any(
        any(feature = "action-schema", feature = "general-schema-section"),
        doc
    ))]
    Action(Action),
}

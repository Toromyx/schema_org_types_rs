use super::*;
/// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
///
/// https://schema.org/potentialAction
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PotentialActionProperty {
    #[cfg(any(feature = "action-schema", feature = "general-schema-section"))]
    Action(Action),
}

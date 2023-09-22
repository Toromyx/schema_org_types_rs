use super::*;
/// A sub property of object. The object that is being replaced.
///
/// https://schema.org/replacee
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReplaceeProperty {
    #[cfg(any(feature = "thing-schema", feature = "general-schema-section"))]
    Thing(Thing),
}
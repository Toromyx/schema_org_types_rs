use super::*;
/// Indicates the primary entity described in some page or other CreativeWork.
///
/// https://schema.org/mainEntity
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MainEntityProperty {
    #[cfg(any(feature = "thing-schema", feature = "general-schema-section"))]
    Thing(Thing),
}

use super::*;
/// Indicates the primary entity described in some page or other CreativeWork.
///
/// <https://schema.org/mainEntity>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MainEntityProperty {
    #[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
    Thing(Thing),
}

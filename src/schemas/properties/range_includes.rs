use super::*;
/// <https://schema.org/rangeIncludes>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RangeIncludesProperty {
    #[cfg(any(any(feature = "class-schema", feature = "meta-schema-section"), doc))]
    Class(Class),
}

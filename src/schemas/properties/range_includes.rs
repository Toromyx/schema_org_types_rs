use super::*;
/// <https://schema.org/rangeIncludes>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RangeIncludesProperty {
    #[cfg(any(any(feature = "class-schema", feature = "meta-schema-section"), doc))]
    Class(Class),
}

use super::*;
/// Relates a property to a class that constitutes (one of) the expected type(s) for values of the property.
///
/// https://schema.org/rangeIncludes
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RangeIncludesProperty {
    #[cfg(any(feature = "class-schema", feature = "meta-schema-section"))]
    Class(Class),
}

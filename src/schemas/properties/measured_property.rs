use super::*;
/// <https://schema.org/measuredProperty>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MeasuredPropertyProperty {
    #[cfg(any(any(feature = "property-schema", feature = "meta-schema-section"), doc))]
    Property(Property),
}

use super::*;
/// <https://schema.org/inverseOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InverseOfProperty {
    #[cfg(any(any(feature = "property-schema", feature = "meta-schema-section"), doc))]
    Property(Property),
}

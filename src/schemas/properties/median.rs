use super::*;
/// The median value.
///
/// https://schema.org/median
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MedianProperty {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}

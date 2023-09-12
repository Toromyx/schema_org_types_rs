use super::*;
/// The 10th percentile value.
///
/// https://schema.org/percentile10
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Percentile10Property {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}

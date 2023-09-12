use super::*;
/// The 90th percentile value.
///
/// https://schema.org/percentile90
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Percentile90Property {
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}

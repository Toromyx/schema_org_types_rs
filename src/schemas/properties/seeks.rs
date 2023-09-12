use super::*;
/// A pointer to products or services sought by the organization or person (demand).
///
/// https://schema.org/seeks
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SeeksProperty {
    #[cfg(any(feature = "demand-schema", feature = "general-schema-section"))]
    Demand(Demand),
}

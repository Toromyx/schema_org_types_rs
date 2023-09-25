use super::*;
/// A pointer to products or services sought by the organization or person (demand).
///
/// https://schema.org/seeks
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SeeksProperty {
    #[cfg(any(
        any(feature = "demand-schema", feature = "general-schema-section"),
        doc
    ))]
    Demand(Demand),
}

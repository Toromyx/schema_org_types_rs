use super::*;
/// <https://schema.org/tributary>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TributaryProperty {
    #[cfg(any(
        any(
            feature = "anatomical-structure-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    AnatomicalStructure(AnatomicalStructure),
}

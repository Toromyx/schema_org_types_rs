use super::*;
/// <https://schema.org/branch>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BranchProperty {
    #[cfg(any(
        any(
            feature = "anatomical-structure-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    AnatomicalStructure(AnatomicalStructure),
}

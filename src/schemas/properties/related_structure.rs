use super::*;
/// Related anatomical structure(s) that are not part of the system but relate or connect to it, such as vascular bundles associated with an organ system.
///
/// https://schema.org/relatedStructure
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RelatedStructureProperty {
    #[cfg(any(
        feature = "anatomical-structure-schema",
        feature = "health-lifesci-schema-section"
    ))]
    AnatomicalStructure(AnatomicalStructure),
}

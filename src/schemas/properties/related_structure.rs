use super::*;
/// Related anatomical structure(s) that are not part of the system but relate or connect to it, such as vascular bundles associated with an organ system.
///
/// <https://schema.org/relatedStructure>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RelatedStructureProperty {
    #[cfg(any(
        any(
            feature = "anatomical-structure-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    AnatomicalStructure(AnatomicalStructure),
}

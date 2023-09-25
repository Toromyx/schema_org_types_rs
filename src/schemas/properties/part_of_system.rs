use super::*;
/// The anatomical or organ system that this structure is part of.
///
/// https://schema.org/partOfSystem
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PartOfSystemProperty {
    #[cfg(any(
        any(
            feature = "anatomical-system-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    AnatomicalSystem(AnatomicalSystem),
}

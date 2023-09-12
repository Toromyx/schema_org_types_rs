use super::*;
/// The anatomical or organ system that this structure is part of.
///
/// https://schema.org/partOfSystem
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PartOfSystemProperty {
    #[cfg(any(
        feature = "anatomical-system-schema",
        feature = "health-lifesci-schema-section"
    ))]
    AnatomicalSystem(AnatomicalSystem),
}

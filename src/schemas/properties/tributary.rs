use super::*;
/// The anatomical or organ system that the vein flows into; a larger structure that the vein connects to.
///
/// https://schema.org/tributary
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TributaryProperty {
    #[cfg(any(
        feature = "anatomical-structure-schema",
        feature = "health-lifesci-schema-section"
    ))]
    AnatomicalStructure(AnatomicalStructure),
}

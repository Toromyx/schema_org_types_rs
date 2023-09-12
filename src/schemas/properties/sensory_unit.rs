use super::*;
/// The neurological pathway extension that inputs and sends information to the brain or spinal cord.
///
/// https://schema.org/sensoryUnit
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SensoryUnitProperty {
    #[cfg(any(
        feature = "anatomical-structure-schema",
        feature = "health-lifesci-schema-section"
    ))]
    AnatomicalStructure(AnatomicalStructure),
    #[cfg(any(
        feature = "superficial-anatomy-schema",
        feature = "health-lifesci-schema-section"
    ))]
    SuperficialAnatomy(SuperficialAnatomy),
}

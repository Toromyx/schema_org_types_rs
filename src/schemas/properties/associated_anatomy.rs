use super::*;
/// The anatomy of the underlying organ system or structures associated with this entity.
///
/// https://schema.org/associatedAnatomy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssociatedAnatomyProperty {
    #[cfg(any(
        feature = "anatomical-structure-schema",
        feature = "health-lifesci-schema-section"
    ))]
    AnatomicalStructure(AnatomicalStructure),
    #[cfg(any(
        feature = "anatomical-system-schema",
        feature = "health-lifesci-schema-section"
    ))]
    AnatomicalSystem(AnatomicalSystem),
    #[cfg(any(
        feature = "superficial-anatomy-schema",
        feature = "health-lifesci-schema-section"
    ))]
    SuperficialAnatomy(SuperficialAnatomy),
}

use super::*;
/// The neurological pathway extension that inputs and sends information to the brain or spinal cord.
///
/// https://schema.org/sensoryUnit
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SensoryUnitProperty {
    #[cfg(any(
        any(
            feature = "anatomical-structure-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    AnatomicalStructure(AnatomicalStructure),
    #[cfg(any(
        any(
            feature = "superficial-anatomy-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    SuperficialAnatomy(SuperficialAnatomy),
}

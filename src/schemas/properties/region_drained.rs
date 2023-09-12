use super::*;
/// The anatomical or organ system drained by this vessel; generally refers to a specific part of an organ.
///
/// https://schema.org/regionDrained
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RegionDrainedProperty {
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
}

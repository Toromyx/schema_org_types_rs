use super::*;
/// Tissue, organ, biological sample, etc in which activity of this gene has been observed experimentally. For example brain, digestive system.
///
/// https://schema.org/expressedIn
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExpressedInProperty {
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
    #[cfg(any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"))]
    BioChemEntity(BioChemEntity),
    #[cfg(any(feature = "defined-term-schema", feature = "pending-schema-section"))]
    DefinedTerm(DefinedTerm),
}

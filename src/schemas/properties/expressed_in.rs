use super::*;
/// <https://schema.org/expressedIn>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExpressedInProperty {
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
            feature = "anatomical-system-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    AnatomicalSystem(AnatomicalSystem),
    #[cfg(any(
        any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
        doc
    ))]
    BioChemEntity(BioChemEntity),
    #[cfg(any(
        any(feature = "defined-term-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedTerm(DefinedTerm),
}

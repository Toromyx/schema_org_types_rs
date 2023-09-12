use super::*;
/// A role played by the BioChemEntity within a chemical context.
///
/// https://schema.org/chemicalRole
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ChemicalRoleProperty {
    #[cfg(any(feature = "defined-term-schema", feature = "pending-schema-section"))]
    DefinedTerm(DefinedTerm),
}

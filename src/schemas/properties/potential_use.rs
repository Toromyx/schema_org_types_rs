use super::*;
/// Intended use of the BioChemEntity by humans.
///
/// https://schema.org/potentialUse
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PotentialUseProperty {
    #[cfg(any(feature = "defined-term-schema", feature = "pending-schema-section"))]
    DefinedTerm(DefinedTerm),
}

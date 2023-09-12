use super::*;
/// Another BioChemEntity encoded by this one.
///
/// https://schema.org/encodesBioChemEntity
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EncodesBioChemEntityProperty {
    #[cfg(any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"))]
    BioChemEntity(BioChemEntity),
}

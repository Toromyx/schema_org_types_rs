use super::*;
/// Indicates a BioChemEntity that is (in some sense) a part of this BioChemEntity.
///
/// https://schema.org/isPartOfBioChemEntity
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsPartOfBioChemEntityProperty {
    #[cfg(any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"))]
    BioChemEntity(BioChemEntity),
}

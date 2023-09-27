use super::*;
/// <https://schema.org/encodesBioChemEntity>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EncodesBioChemEntityProperty {
    #[cfg(any(
        any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
        doc
    ))]
    BioChemEntity(BioChemEntity),
}

use super::*;
/// Another BioChemEntity encoding by this one.
///
/// <https://schema.org/isEncodedByBioChemEntity>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IsEncodedByBioChemEntityProperty {
    #[cfg(any(any(feature = "gene-schema", feature = "pending-schema-section"), doc))]
    Gene(Gene),
}

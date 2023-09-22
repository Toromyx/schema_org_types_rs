use super::*;
/// A BioChemEntity that is known to interact with this item.
///
/// https://schema.org/bioChemInteraction
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BioChemInteractionProperty {
    #[cfg(any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"))]
    BioChemEntity(BioChemEntity),
}
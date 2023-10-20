use super::*;
/// <https://schema.org/isPartOfBioChemEntity>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsPartOfBioChemEntityProperty {
	#[cfg(any(
		any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
		doc
	))]
	BioChemEntity(BioChemEntity),
}

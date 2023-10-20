use super::*;
/// <https://schema.org/fundedItem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FundedItemProperty {
	#[cfg(any(
		any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
		doc
	))]
	BioChemEntity(BioChemEntity),
	#[cfg(any(
		any(feature = "creative-work-schema", feature = "general-schema-section"),
		doc
	))]
	CreativeWork(CreativeWork),
	#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
	Event(Event),
	#[cfg(any(
		any(
			feature = "medical-entity-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalEntity(MedicalEntity),
	#[cfg(any(
		any(feature = "organization-schema", feature = "general-schema-section"),
		doc
	))]
	Organization(Organization),
	#[cfg(any(
		any(feature = "person-schema", feature = "general-schema-section"),
		doc
	))]
	Person(Person),
	#[cfg(any(
		any(feature = "product-schema", feature = "general-schema-section"),
		doc
	))]
	Product(Product),
}

use super::*;
/// Indicates something directly or indirectly funded or sponsored through a [[Grant]]. See also [[ownershipFundingInfo]].
///
/// https://schema.org/fundedItem
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FundedItemProperty {
    #[cfg(any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"))]
    BioChemEntity(BioChemEntity),
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "event-schema", feature = "general-schema-section"))]
    Event(Event),
    #[cfg(any(
        feature = "medical-entity-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalEntity(MedicalEntity),
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
    #[cfg(any(feature = "person-schema", feature = "general-schema-section"))]
    Person(Person),
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
}

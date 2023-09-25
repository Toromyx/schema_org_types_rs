use super::*;
/// Indicates something directly or indirectly funded or sponsored through a [[Grant]]. See also [[ownershipFundingInfo]].
///
/// <https://schema.org/fundedItem>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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

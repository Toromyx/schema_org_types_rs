use super::*;
/// <https://schema.org/eligibleCustomerType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EligibleCustomerTypeProperty {
    #[cfg(any(
        any(
            feature = "business-entity-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BusinessEntityType(BusinessEntityType),
}

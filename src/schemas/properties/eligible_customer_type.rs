use super::*;
/// The type(s) of customers for which the given offer is valid.
///
/// https://schema.org/eligibleCustomerType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EligibleCustomerTypeProperty {
    #[cfg(any(
        feature = "business-entity-type-schema",
        feature = "general-schema-section"
    ))]
    BusinessEntityType(BusinessEntityType),
}

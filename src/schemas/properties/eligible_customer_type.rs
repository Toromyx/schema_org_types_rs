use super::*;
/// <https://schema.org/eligibleCustomerType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

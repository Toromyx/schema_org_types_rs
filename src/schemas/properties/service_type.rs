use super::*;
/// <https://schema.org/serviceType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ServiceTypeProperty {
	#[cfg(any(
		any(
			feature = "government-benefits-type-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	GovernmentBenefitsType(GovernmentBenefitsType),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}

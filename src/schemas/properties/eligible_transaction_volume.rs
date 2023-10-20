use super::*;
/// <https://schema.org/eligibleTransactionVolume>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EligibleTransactionVolumeProperty {
	#[cfg(any(
		any(
			feature = "price-specification-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	PriceSpecification(PriceSpecification),
}

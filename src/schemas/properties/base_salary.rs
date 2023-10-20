use super::*;
/// <https://schema.org/baseSalary>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BaseSalaryProperty {
	#[cfg(any(
		any(feature = "monetary-amount-schema", feature = "general-schema-section"),
		doc
	))]
	MonetaryAmount(MonetaryAmount),
	#[cfg(any(
		any(
			feature = "price-specification-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	PriceSpecification(PriceSpecification),
	#[cfg(any(
		any(feature = "number-schema", feature = "general-schema-section"),
		doc
	))]
	Number(Number),
}

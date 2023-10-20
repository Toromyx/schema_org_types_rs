use super::*;
/// <https://schema.org/depth>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DepthProperty {
	#[cfg(any(
		any(feature = "distance-schema", feature = "general-schema-section"),
		doc
	))]
	Distance(Distance),
	#[cfg(any(
		any(
			feature = "quantitative-value-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	QuantitativeValue(QuantitativeValue),
}

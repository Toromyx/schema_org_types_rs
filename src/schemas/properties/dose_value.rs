use super::*;
/// <https://schema.org/doseValue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DoseValueProperty {
	#[cfg(any(
		any(
			feature = "qualitative-value-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	QualitativeValue(QualitativeValue),
	#[cfg(any(
		any(feature = "number-schema", feature = "general-schema-section"),
		doc
	))]
	Number(Number),
}

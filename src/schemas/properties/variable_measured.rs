use super::*;
/// <https://schema.org/variableMeasured>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VariableMeasuredProperty {
	#[cfg(any(any(feature = "property-schema", feature = "meta-schema-section"), doc))]
	Property(Property),
	#[cfg(any(
		any(feature = "property-value-schema", feature = "general-schema-section"),
		doc
	))]
	PropertyValue(PropertyValue),
	#[cfg(any(
		any(
			feature = "statistical-variable-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	StatisticalVariable(StatisticalVariable),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}

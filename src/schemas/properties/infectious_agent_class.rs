use super::*;
/// <https://schema.org/infectiousAgentClass>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InfectiousAgentClassProperty {
	#[cfg(any(
		any(
			feature = "infectious-agent-class-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	InfectiousAgentClass(InfectiousAgentClass),
}

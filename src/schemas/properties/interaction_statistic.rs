use super::*;
/// <https://schema.org/interactionStatistic>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InteractionStatisticProperty {
	#[cfg(any(
		any(
			feature = "interaction-counter-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	InteractionCounter(InteractionCounter),
}

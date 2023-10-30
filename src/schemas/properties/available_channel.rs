use super::*;
/// <https://schema.org/availableChannel>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableChannelProperty {
	#[cfg(any(
		any(feature = "service-channel-schema", feature = "general-schema-section"),
		doc
	))]
	ServiceChannel(ServiceChannel),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}

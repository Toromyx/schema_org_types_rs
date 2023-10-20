use super::*;
/// <https://schema.org/actionStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ActionStatusProperty {
	#[cfg(any(
		any(
			feature = "action-status-type-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	ActionStatusType(ActionStatusType),
}

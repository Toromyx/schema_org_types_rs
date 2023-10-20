use super::*;
/// <https://schema.org/free>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FreeProperty {
	#[cfg(any(
		any(feature = "boolean-schema", feature = "general-schema-section"),
		doc
	))]
	Boolean(Boolean),
}

use super::*;
/// <https://schema.org/returnLabelSource>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReturnLabelSourceProperty {
	#[cfg(any(
		any(
			feature = "return-label-source-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	ReturnLabelSourceEnumeration(ReturnLabelSourceEnumeration),
}

use super::*;
/// <https://schema.org/hasAdultConsideration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasAdultConsiderationProperty {
	#[cfg(any(
		any(
			feature = "adult-oriented-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	AdultOrientedEnumeration(AdultOrientedEnumeration),
}

use super::*;
/// <https://schema.org/nonprofitStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NonprofitStatusProperty {
	#[cfg(any(
		any(feature = "nonprofit-type-schema", feature = "pending-schema-section"),
		doc
	))]
	NonprofitType(NonprofitType),
}

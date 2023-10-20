use super::*;
/// <https://schema.org/boardingPolicy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BoardingPolicyProperty {
	#[cfg(any(
		any(
			feature = "boarding-policy-type-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	BoardingPolicyType(BoardingPolicyType),
}

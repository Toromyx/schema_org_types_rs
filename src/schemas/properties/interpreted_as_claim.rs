use super::*;
/// <https://schema.org/interpretedAsClaim>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InterpretedAsClaimProperty {
	#[cfg(any(any(feature = "claim-schema", feature = "pending-schema-section"), doc))]
	Claim(Claim),
}

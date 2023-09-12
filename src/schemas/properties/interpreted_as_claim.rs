use super::*;
/// Used to indicate a specific claim contained, implied, translated or refined from the content of a [[MediaObject]] or other [[CreativeWork]]. The interpreting party can be indicated using [[claimInterpreter]].
///
/// https://schema.org/interpretedAsClaim
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InterpretedAsClaimProperty {
    #[cfg(any(feature = "claim-schema", feature = "pending-schema-section"))]
    Claim(Claim),
}

use super::*;
/// Used to indicate a specific claim contained, implied, translated or refined from the content of a [[MediaObject]] or other [[CreativeWork]]. The interpreting party can be indicated using [[claimInterpreter]].
///
/// <https://schema.org/interpretedAsClaim>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InterpretedAsClaimProperty {
    #[cfg(any(any(feature = "claim-schema", feature = "pending-schema-section"), doc))]
    Claim(Claim),
}

use super::*;
/// <https://schema.org/expectsAcceptanceOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExpectsAcceptanceOfProperty {
    #[cfg(any(any(feature = "offer-schema", feature = "general-schema-section"), doc))]
    Offer(Offer),
}

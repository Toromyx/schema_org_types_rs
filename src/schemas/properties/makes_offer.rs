use super::*;
/// <https://schema.org/makesOffer>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MakesOfferProperty {
    #[cfg(any(any(feature = "offer-schema", feature = "general-schema-section"), doc))]
    Offer(Offer),
}

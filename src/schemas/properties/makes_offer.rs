use super::*;
/// A pointer to products or services offered by the organization or person.
///
/// https://schema.org/makesOffer
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MakesOfferProperty {
    #[cfg(any(feature = "offer-schema", feature = "general-schema-section"))]
    Offer(Offer),
}

use super::*;
/// The offer(s) -- e.g., product, quantity and price combinations -- included in the order.
///
/// https://schema.org/acceptedOffer
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AcceptedOfferProperty {
    #[cfg(any(feature = "offer-schema", feature = "general-schema-section"))]
    Offer(Offer),
}

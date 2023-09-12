use super::*;
/// Indicates information about the shipping policies and options associated with an [[Offer]].
///
/// https://schema.org/shippingDetails
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ShippingDetailsProperty {
    #[cfg(any(
        feature = "offer-shipping-details-schema",
        feature = "pending-schema-section"
    ))]
    OfferShippingDetails(OfferShippingDetails),
}

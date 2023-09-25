use super::*;
/// Indicates information about the shipping policies and options associated with an [[Offer]].
///
/// https://schema.org/shippingDetails
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ShippingDetailsProperty {
    #[cfg(any(
        any(
            feature = "offer-shipping-details-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    OfferShippingDetails(OfferShippingDetails),
}

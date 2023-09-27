use super::*;
/// <https://schema.org/shippingDetails>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

use super::*;
/// A monetary value above (or at) which the shipping rate becomes free. Intended to be used via an [[OfferShippingDetails]] with [[shippingSettingsLink]] matching this [[ShippingRateSettings]].
///
/// https://schema.org/freeShippingThreshold
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum FreeShippingThresholdProperty {
    #[cfg(any(
        any(
            feature = "delivery-charge-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    DeliveryChargeSpecification(DeliveryChargeSpecification),
    #[cfg(any(
        any(feature = "monetary-amount-schema", feature = "general-schema-section"),
        doc
    ))]
    MonetaryAmount(MonetaryAmount),
}

use super::*;
/// A monetary value above (or at) which the shipping rate becomes free. Intended to be used via an [[OfferShippingDetails]] with [[shippingSettingsLink]] matching this [[ShippingRateSettings]].
///
/// https://schema.org/freeShippingThreshold
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FreeShippingThresholdProperty {
    #[cfg(any(
        feature = "delivery-charge-specification-schema",
        feature = "general-schema-section"
    ))]
    DeliveryChargeSpecification(DeliveryChargeSpecification),
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
}

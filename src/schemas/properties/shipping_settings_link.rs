use super::*;
/// Link to a page containing [[ShippingRateSettings]] and [[DeliveryTimeSettings]] details.
///
/// https://schema.org/shippingSettingsLink
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ShippingSettingsLinkProperty {
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}

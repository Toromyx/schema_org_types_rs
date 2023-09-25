use super::*;
/// A URL template (RFC 6570) for a checkout page for an offer. This approach allows merchants to specify a URL for online checkout of the offered product, by interpolating parameters such as the logged in user ID, product ID, quantity, discount code etc. Parameter naming and standardization are not specified here.
///
/// <https://schema.org/checkoutPageURLTemplate>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CheckoutPageUrlTemplateProperty {
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}

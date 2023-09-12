use super::*;
/// The billing address for the order.
///
/// https://schema.org/billingAddress
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BillingAddressProperty {
    #[cfg(any(feature = "postal-address-schema", feature = "general-schema-section"))]
    PostalAddress(PostalAddress),
}

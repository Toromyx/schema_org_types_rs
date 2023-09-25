use super::*;
/// The payment method(s) to which the payment charge specification applies.
///
/// https://schema.org/appliesToPaymentMethod
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AppliesToPaymentMethodProperty {
    #[cfg(any(
        any(feature = "payment-method-schema", feature = "general-schema-section"),
        doc
    ))]
    PaymentMethod(PaymentMethod),
}

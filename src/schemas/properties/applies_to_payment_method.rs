use super::*;
/// The payment method(s) to which the payment charge specification applies.
///
/// https://schema.org/appliesToPaymentMethod
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AppliesToPaymentMethodProperty {
    #[cfg(any(feature = "payment-method-schema", feature = "general-schema-section"))]
    PaymentMethod(PaymentMethod),
}

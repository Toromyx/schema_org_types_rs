use super::*;
/// The name of the credit card or other method of payment for the order.
///
/// https://schema.org/paymentMethod
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PaymentMethodProperty {
    #[cfg(any(feature = "payment-method-schema", feature = "general-schema-section"))]
    PaymentMethod(PaymentMethod),
}

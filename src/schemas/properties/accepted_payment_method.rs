use super::*;
/// The payment method(s) accepted by seller for this offer.
///
/// https://schema.org/acceptedPaymentMethod
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AcceptedPaymentMethodProperty {
    #[cfg(any(feature = "loan-or-credit-schema", feature = "general-schema-section"))]
    LoanOrCredit(LoanOrCredit),
    #[cfg(any(feature = "payment-method-schema", feature = "general-schema-section"))]
    PaymentMethod(PaymentMethod),
}

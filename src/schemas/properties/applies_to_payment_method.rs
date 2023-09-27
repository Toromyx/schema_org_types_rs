use super::*;
/// <https://schema.org/appliesToPaymentMethod>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AppliesToPaymentMethodProperty {
    #[cfg(any(
        any(feature = "payment-method-schema", feature = "general-schema-section"),
        doc
    ))]
    PaymentMethod(PaymentMethod),
}

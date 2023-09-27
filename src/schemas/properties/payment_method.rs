use super::*;
/// <https://schema.org/paymentMethod>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PaymentMethodProperty {
    #[cfg(any(
        any(feature = "payment-method-schema", feature = "general-schema-section"),
        doc
    ))]
    PaymentMethod(PaymentMethod),
}

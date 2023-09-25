use super::*;
/// The status of payment; whether the invoice has been paid or not.
///
/// https://schema.org/paymentStatus
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PaymentStatusProperty {
    #[cfg(any(
        any(
            feature = "payment-status-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    PaymentStatusType(PaymentStatusType),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}

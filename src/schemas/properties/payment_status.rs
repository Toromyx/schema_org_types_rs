use super::*;
/// <https://schema.org/paymentStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

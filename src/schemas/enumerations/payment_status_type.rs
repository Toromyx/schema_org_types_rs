/// <https://schema.org/PaymentStatusType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PaymentStatusType {
    /// <https://schema.org/PaymentAutomaticallyApplied>
    PaymentAutomaticallyApplied,
    /// <https://schema.org/PaymentComplete>
    PaymentComplete,
    /// <https://schema.org/PaymentDeclined>
    PaymentDeclined,
    /// <https://schema.org/PaymentDue>
    PaymentDue,
    /// <https://schema.org/PaymentPastDue>
    PaymentPastDue,
}

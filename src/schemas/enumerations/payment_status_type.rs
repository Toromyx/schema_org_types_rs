/// A specific payment status. For example, PaymentDue, PaymentComplete, etc.
///
/// https://schema.org/PaymentStatusType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PaymentStatusType {
    /// An automatic payment system is in place and will be used.
    ///
    /// https://schema.org/PaymentAutomaticallyApplied
    PaymentAutomaticallyApplied,
    /// The payment has been received and processed.
    ///
    /// https://schema.org/PaymentComplete
    PaymentComplete,
    /// The payee received the payment, but it was declined for some reason.
    ///
    /// https://schema.org/PaymentDeclined
    PaymentDeclined,
    /// The payment is due, but still within an acceptable time to be received.
    ///
    /// https://schema.org/PaymentDue
    PaymentDue,
    /// The payment is due and considered late.
    ///
    /// https://schema.org/PaymentPastDue
    PaymentPastDue,
}

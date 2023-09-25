/// A payment method using a credit, debit, store or other card to associate the payment with an account.
///
/// <https://schema.org/PaymentCard>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PaymentCard {}

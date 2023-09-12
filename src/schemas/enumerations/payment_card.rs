/// A payment method using a credit, debit, store or other card to associate the payment with an account.
///
/// https://schema.org/PaymentCard
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PaymentCard {}

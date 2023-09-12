use super::*;
/// The minimum payment is the lowest amount of money that one is required to pay on a credit card statement each month.
///
/// https://schema.org/monthlyMinimumRepaymentAmount
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MonthlyMinimumRepaymentAmountProperty {
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}

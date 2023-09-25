use super::*;
/// The minimum payment is the lowest amount of money that one is required to pay on a credit card statement each month.
///
/// <https://schema.org/monthlyMinimumRepaymentAmount>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MonthlyMinimumRepaymentAmountProperty {
    #[cfg(any(
        any(feature = "monetary-amount-schema", feature = "general-schema-section"),
        doc
    ))]
    MonetaryAmount(MonetaryAmount),
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
}

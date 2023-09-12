use super::*;
/// A minimum amount that has to be paid in every month.
///
/// https://schema.org/accountMinimumInflow
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AccountMinimumInflowProperty {
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
}

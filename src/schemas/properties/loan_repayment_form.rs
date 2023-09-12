use super::*;
/// A form of paying back money previously borrowed from a lender. Repayment usually takes the form of periodic payments that normally include part principal plus interest in each payment.
///
/// https://schema.org/loanRepaymentForm
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LoanRepaymentFormProperty {
    #[cfg(any(
        feature = "repayment-specification-schema",
        feature = "pending-schema-section"
    ))]
    RepaymentSpecification(RepaymentSpecification),
}

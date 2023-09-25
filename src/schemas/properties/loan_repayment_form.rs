use super::*;
/// A form of paying back money previously borrowed from a lender. Repayment usually takes the form of periodic payments that normally include part principal plus interest in each payment.
///
/// <https://schema.org/loanRepaymentForm>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LoanRepaymentFormProperty {
    #[cfg(any(
        any(
            feature = "repayment-specification-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    RepaymentSpecification(RepaymentSpecification),
}

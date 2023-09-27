use super::*;
/// <https://schema.org/loanRepaymentForm>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

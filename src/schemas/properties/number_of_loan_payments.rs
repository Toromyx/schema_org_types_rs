use super::*;
/// The number of payments contractually required at origination to repay the loan. For monthly paying loans this is the number of months from the contractual first payment date to the maturity date.
///
/// <https://schema.org/numberOfLoanPayments>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NumberOfLoanPaymentsProperty {
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
}

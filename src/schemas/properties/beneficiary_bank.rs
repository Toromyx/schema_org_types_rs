use super::*;
/// A bank or bank’s branch, financial institution or international financial institution operating the beneficiary’s bank account or releasing funds for the beneficiary.
///
/// https://schema.org/beneficiaryBank
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BeneficiaryBankProperty {
    #[cfg(any(
        feature = "bank-or-credit-union-schema",
        feature = "general-schema-section"
    ))]
    BankOrCreditUnion(BankOrCreditUnion),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}

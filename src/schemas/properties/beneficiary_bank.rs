use super::*;
/// A bank or bank’s branch, financial institution or international financial institution operating the beneficiary’s bank account or releasing funds for the beneficiary.
///
/// https://schema.org/beneficiaryBank
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BeneficiaryBankProperty {
    #[cfg(any(
        any(
            feature = "bank-or-credit-union-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BankOrCreditUnion(BankOrCreditUnion),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}

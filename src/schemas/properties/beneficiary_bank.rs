use super::*;
/// <https://schema.org/beneficiaryBank>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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

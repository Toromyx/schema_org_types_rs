use super::*;
/// A financial aid type or program which students may use to pay for tuition or fees associated with the program.
///
/// https://schema.org/financialAidEligible
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FinancialAidEligibleProperty {
    #[cfg(any(feature = "defined-term-schema", feature = "pending-schema-section"))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}

use super::*;
/// <https://schema.org/accountOverdraftLimit>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AccountOverdraftLimitProperty {
    #[cfg(any(
        any(feature = "monetary-amount-schema", feature = "general-schema-section"),
        doc
    ))]
    MonetaryAmount(MonetaryAmount),
}

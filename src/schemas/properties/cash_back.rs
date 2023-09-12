use super::*;
/// A cardholder benefit that pays the cardholder a small percentage of their net expenditures.
///
/// https://schema.org/cashBack
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CashBackProperty {
    #[cfg(any(feature = "boolean-schema", feature = "general-schema-section"))]
    Boolean(Boolean),
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}

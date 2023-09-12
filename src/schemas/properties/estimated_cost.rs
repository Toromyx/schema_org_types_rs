use super::*;
/// The estimated cost of the supply or supplies consumed when performing instructions.
///
/// https://schema.org/estimatedCost
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum EstimatedCostProperty {
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}

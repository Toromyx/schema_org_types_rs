use super::*;
/// Amount of shipping costs for defect product returns. Applicable when property [[itemDefectReturnFees]] equals [[ReturnShippingFees]].
///
/// https://schema.org/itemDefectReturnShippingFeesAmount
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemDefectReturnShippingFeesAmountProperty {
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
}

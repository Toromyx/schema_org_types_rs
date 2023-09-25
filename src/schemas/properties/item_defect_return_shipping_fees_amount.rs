use super::*;
/// Amount of shipping costs for defect product returns. Applicable when property [[itemDefectReturnFees]] equals [[ReturnShippingFees]].
///
/// <https://schema.org/itemDefectReturnShippingFeesAmount>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ItemDefectReturnShippingFeesAmountProperty {
    #[cfg(any(
        any(feature = "monetary-amount-schema", feature = "general-schema-section"),
        doc
    ))]
    MonetaryAmount(MonetaryAmount),
}

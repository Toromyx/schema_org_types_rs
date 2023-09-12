use super::*;
/// The amount of shipping costs if a product is returned due to customer remorse. Applicable when property [[customerRemorseReturnFees]] equals [[ReturnShippingFees]].
///
/// https://schema.org/customerRemorseReturnShippingFeesAmount
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CustomerRemorseReturnShippingFeesAmountProperty {
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
}

use super::*;
/// The type of return fees if the product is returned due to customer remorse.
///
/// https://schema.org/customerRemorseReturnFees
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CustomerRemorseReturnFeesProperty {
    #[cfg(any(
        feature = "return-fees-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    ReturnFeesEnumeration(ReturnFeesEnumeration),
}

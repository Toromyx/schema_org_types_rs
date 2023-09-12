use super::*;
/// The type of return fees for purchased products (for any return reason).
///
/// https://schema.org/returnFees
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReturnFeesProperty {
    #[cfg(any(
        feature = "return-fees-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    ReturnFeesEnumeration(ReturnFeesEnumeration),
}

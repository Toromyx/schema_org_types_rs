use super::*;
/// Specifies an applicable return policy (from an enumeration).
///
/// https://schema.org/returnPolicyCategory
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReturnPolicyCategoryProperty {
    #[cfg(any(
        feature = "merchant-return-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    MerchantReturnEnumeration(MerchantReturnEnumeration),
}

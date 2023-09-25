use super::*;
/// Specifies an applicable return policy (from an enumeration).
///
/// https://schema.org/returnPolicyCategory
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ReturnPolicyCategoryProperty {
    #[cfg(any(
        any(
            feature = "merchant-return-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    MerchantReturnEnumeration(MerchantReturnEnumeration),
}

use super::*;
/// Specifies a MerchantReturnPolicy that may be applicable.
///
/// https://schema.org/hasMerchantReturnPolicy
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasMerchantReturnPolicyProperty {
    #[cfg(any(
        any(
            feature = "merchant-return-policy-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    MerchantReturnPolicy(MerchantReturnPolicy),
}

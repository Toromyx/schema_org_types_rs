use super::*;
/// Indicates a ProductReturnPolicy that may be applicable.
///
/// https://schema.org/hasProductReturnPolicy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasProductReturnPolicyProperty {
    #[cfg(any(
        feature = "product-return-policy-schema",
        feature = "attic-schema-section"
    ))]
    ProductReturnPolicy(ProductReturnPolicy),
}

use super::*;
/// Indicates a ProductReturnPolicy that may be applicable.
///
/// https://schema.org/hasProductReturnPolicy
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasProductReturnPolicyProperty {
    #[cfg(any(
        any(
            feature = "product-return-policy-schema",
            feature = "attic-schema-section"
        ),
        doc
    ))]
    ProductReturnPolicy(ProductReturnPolicy),
}

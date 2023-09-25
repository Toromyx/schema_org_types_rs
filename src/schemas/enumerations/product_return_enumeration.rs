/// ProductReturnEnumeration enumerates several kinds of product return policy. Note that this structure may not capture all aspects of the policy.
///
/// https://schema.org/ProductReturnEnumeration
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ProductReturnEnumeration {
    /// ProductReturnFiniteReturnWindow: there is a finite window for product returns.
    ///
    /// https://schema.org/ProductReturnFiniteReturnWindow
    ProductReturnFiniteReturnWindow,
    /// ProductReturnNotPermitted: product returns are not permitted.
    ///
    /// https://schema.org/ProductReturnNotPermitted
    ProductReturnNotPermitted,
    /// ProductReturnUnlimitedWindow: there is an unlimited window for product returns.
    ///
    /// https://schema.org/ProductReturnUnlimitedWindow
    ProductReturnUnlimitedWindow,
    /// ProductReturnUnspecified: a product return policy is not specified here.
    ///
    /// https://schema.org/ProductReturnUnspecified
    ProductReturnUnspecified,
}

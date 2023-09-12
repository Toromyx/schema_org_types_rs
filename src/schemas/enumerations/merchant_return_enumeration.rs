/// Enumerates several kinds of product return policies.
///
/// https://schema.org/MerchantReturnEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MerchantReturnEnumeration {
    /// Specifies that there is a finite window for product returns.
    ///
    /// https://schema.org/MerchantReturnFiniteReturnWindow
    MerchantReturnFiniteReturnWindow,
    /// Specifies that product returns are not permitted.
    ///
    /// https://schema.org/MerchantReturnNotPermitted
    MerchantReturnNotPermitted,
    /// Specifies that there is an unlimited window for product returns.
    ///
    /// https://schema.org/MerchantReturnUnlimitedWindow
    MerchantReturnUnlimitedWindow,
    /// Specifies that a product return policy is not provided.
    ///
    /// https://schema.org/MerchantReturnUnspecified
    MerchantReturnUnspecified,
}

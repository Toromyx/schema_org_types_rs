/// <https://schema.org/MerchantReturnEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MerchantReturnEnumeration {
    /// <https://schema.org/MerchantReturnFiniteReturnWindow>
    MerchantReturnFiniteReturnWindow,
    /// <https://schema.org/MerchantReturnNotPermitted>
    MerchantReturnNotPermitted,
    /// <https://schema.org/MerchantReturnUnlimitedWindow>
    MerchantReturnUnlimitedWindow,
    /// <https://schema.org/MerchantReturnUnspecified>
    MerchantReturnUnspecified,
}

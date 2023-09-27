/// <https://schema.org/ProductReturnEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProductReturnEnumeration {
    /// <https://schema.org/ProductReturnFiniteReturnWindow>
    ProductReturnFiniteReturnWindow,
    /// <https://schema.org/ProductReturnNotPermitted>
    ProductReturnNotPermitted,
    /// <https://schema.org/ProductReturnUnlimitedWindow>
    ProductReturnUnlimitedWindow,
    /// <https://schema.org/ProductReturnUnspecified>
    ProductReturnUnspecified,
}

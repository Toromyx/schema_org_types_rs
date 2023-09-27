/// <https://schema.org/ProductReturnEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

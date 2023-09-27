/// <https://schema.org/PriceTypeEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PriceTypeEnumeration {
    /// <https://schema.org/InvoicePrice>
    InvoicePrice,
    /// <https://schema.org/ListPrice>
    ListPrice,
    /// <https://schema.org/MSRP>
    Msrp,
    /// <https://schema.org/MinimumAdvertisedPrice>
    MinimumAdvertisedPrice,
    /// <https://schema.org/SRP>
    Srp,
    /// <https://schema.org/SalePrice>
    SalePrice,
}

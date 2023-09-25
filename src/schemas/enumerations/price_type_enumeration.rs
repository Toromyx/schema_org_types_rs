/// Enumerates different price types, for example list price, invoice price, and sale price.
///
/// <https://schema.org/PriceTypeEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PriceTypeEnumeration {
    /// Represents the invoice price of an offered product.
    ///
    /// <https://schema.org/InvoicePrice>
    InvoicePrice,
    /// Represents the list price (the price a product is actually advertised for) of an offered product.
    ///
    /// <https://schema.org/ListPrice>
    ListPrice,
    /// Represents the manufacturer suggested retail price ("MSRP") of an offered product.
    ///
    /// <https://schema.org/MSRP>
    Msrp,
    /// Represents the minimum advertised price ("MAP") (as dictated by the manufacturer) of an offered product.
    ///
    /// <https://schema.org/MinimumAdvertisedPrice>
    MinimumAdvertisedPrice,
    /// Represents the suggested retail price ("SRP") of an offered product.
    ///
    /// <https://schema.org/SRP>
    Srp,
    /// Represents a sale price (usually active for a limited period) of an offered product.
    ///
    /// <https://schema.org/SalePrice>
    SalePrice,
}

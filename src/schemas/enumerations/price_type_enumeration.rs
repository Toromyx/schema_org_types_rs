/// <https://schema.org/PriceTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// <https://schema.org/ItemAvailability>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ItemAvailability {
	/// <https://schema.org/BackOrder>
	BackOrder,
	/// <https://schema.org/Discontinued>
	Discontinued,
	/// <https://schema.org/InStock>
	InStock,
	/// <https://schema.org/InStoreOnly>
	InStoreOnly,
	/// <https://schema.org/LimitedAvailability>
	LimitedAvailability,
	/// <https://schema.org/OnlineOnly>
	OnlineOnly,
	/// <https://schema.org/OutOfStock>
	OutOfStock,
	/// <https://schema.org/PreOrder>
	PreOrder,
	/// <https://schema.org/PreSale>
	PreSale,
	/// <https://schema.org/SoldOut>
	SoldOut,
}

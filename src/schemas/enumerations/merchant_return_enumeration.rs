/// <https://schema.org/MerchantReturnEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

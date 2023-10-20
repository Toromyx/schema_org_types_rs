/// <https://schema.org/ReturnFeesEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnFeesEnumeration {
	/// <https://schema.org/FreeReturn>
	FreeReturn,
	/// <https://schema.org/OriginalShippingFees>
	OriginalShippingFees,
	/// <https://schema.org/RestockingFees>
	RestockingFees,
	/// <https://schema.org/ReturnFeesCustomerResponsibility>
	ReturnFeesCustomerResponsibility,
	/// <https://schema.org/ReturnShippingFees>
	ReturnShippingFees,
}

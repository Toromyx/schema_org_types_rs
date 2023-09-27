/// <https://schema.org/ReturnFeesEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

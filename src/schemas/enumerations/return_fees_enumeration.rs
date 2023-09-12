/// Enumerates several kinds of policies for product return fees.
///
/// https://schema.org/ReturnFeesEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnFeesEnumeration {
    /// Specifies that product returns are free of charge for the customer.
    ///
    /// https://schema.org/FreeReturn
    FreeReturn,
    /// Specifies that the customer must pay the original shipping costs when returning a product.
    ///
    /// https://schema.org/OriginalShippingFees
    OriginalShippingFees,
    /// Specifies that the customer must pay a restocking fee when returning a product.
    ///
    /// https://schema.org/RestockingFees
    RestockingFees,
    /// Specifies that product returns must be paid for, and are the responsibility of, the customer.
    ///
    /// https://schema.org/ReturnFeesCustomerResponsibility
    ReturnFeesCustomerResponsibility,
    /// Specifies that the customer must pay the return shipping costs when returning a product.
    ///
    /// https://schema.org/ReturnShippingFees
    ReturnShippingFees,
}

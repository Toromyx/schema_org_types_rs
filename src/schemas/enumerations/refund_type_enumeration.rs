/// Enumerates several kinds of product return refund types.
///
/// https://schema.org/RefundTypeEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefundTypeEnumeration {
    /// Specifies that a refund can be done as an exchange for the same product.
    ///
    /// https://schema.org/ExchangeRefund
    ExchangeRefund,
    /// Specifies that a refund can be done in the full amount the customer paid for the product.
    ///
    /// https://schema.org/FullRefund
    FullRefund,
    /// Specifies that the customer receives a store credit as refund when returning a product.
    ///
    /// https://schema.org/StoreCreditRefund
    StoreCreditRefund,
}

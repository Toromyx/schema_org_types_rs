/// Enumerates several kinds of product return refund types.
///
/// https://schema.org/RefundTypeEnumeration
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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

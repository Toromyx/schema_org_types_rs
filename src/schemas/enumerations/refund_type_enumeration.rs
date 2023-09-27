/// <https://schema.org/RefundTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefundTypeEnumeration {
    /// <https://schema.org/ExchangeRefund>
    ExchangeRefund,
    /// <https://schema.org/FullRefund>
    FullRefund,
    /// <https://schema.org/StoreCreditRefund>
    StoreCreditRefund,
}

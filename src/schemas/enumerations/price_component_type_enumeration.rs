/// <https://schema.org/PriceComponentTypeEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PriceComponentTypeEnumeration {
    /// <https://schema.org/ActivationFee>
    ActivationFee,
    /// <https://schema.org/CleaningFee>
    CleaningFee,
    /// <https://schema.org/DistanceFee>
    DistanceFee,
    /// <https://schema.org/Downpayment>
    Downpayment,
    /// <https://schema.org/Installment>
    Installment,
    /// <https://schema.org/Subscription>
    Subscription,
}

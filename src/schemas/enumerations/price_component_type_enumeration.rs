/// <https://schema.org/PriceComponentTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

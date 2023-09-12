/// Enumerates different price components that together make up the total price for an offered product.
///
/// https://schema.org/PriceComponentTypeEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PriceComponentTypeEnumeration {
    /// Represents the activation fee part of the total price for an offered product, for example a cellphone contract.
    ///
    /// https://schema.org/ActivationFee
    ActivationFee,
    /// Represents the cleaning fee part of the total price for an offered product, for example a vacation rental.
    ///
    /// https://schema.org/CleaningFee
    CleaningFee,
    /// Represents the distance fee (e.g., price per km or mile) part of the total price for an offered product, for example a car rental.
    ///
    /// https://schema.org/DistanceFee
    DistanceFee,
    /// Represents the downpayment (up-front payment) price component of the total price for an offered product that has additional installment payments.
    ///
    /// https://schema.org/Downpayment
    Downpayment,
    /// Represents the installment pricing component of the total price for an offered product.
    ///
    /// https://schema.org/Installment
    Installment,
    /// Represents the subscription pricing component of the total price for an offered product.
    ///
    /// https://schema.org/Subscription
    Subscription,
}

use super::*;
/// The minimum payment required at this time.
///
/// https://schema.org/minimumPaymentDue
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MinimumPaymentDueProperty {
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
    #[cfg(any(
        feature = "price-specification-schema",
        feature = "general-schema-section"
    ))]
    PriceSpecification(PriceSpecification),
}

use super::*;
/// The current price of a currency.
///
/// https://schema.org/currentExchangeRate
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CurrentExchangeRateProperty {
    #[cfg(any(
        feature = "unit-price-specification-schema",
        feature = "general-schema-section"
    ))]
    UnitPriceSpecification(UnitPriceSpecification),
}

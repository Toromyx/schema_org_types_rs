use super::*;
/// The difference between the price at which a broker or other intermediary buys and sells foreign currency.
///
/// https://schema.org/exchangeRateSpread
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExchangeRateSpreadProperty {
    #[cfg(any(feature = "monetary-amount-schema", feature = "general-schema-section"))]
    MonetaryAmount(MonetaryAmount),
    #[cfg(any(feature = "number-schema", feature = "general-schema-section"))]
    Number(Number),
}

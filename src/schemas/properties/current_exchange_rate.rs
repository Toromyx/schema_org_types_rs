use super::*;
/// The current price of a currency.
///
/// <https://schema.org/currentExchangeRate>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CurrentExchangeRateProperty {
    #[cfg(any(
        any(
            feature = "unit-price-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    UnitPriceSpecification(UnitPriceSpecification),
}

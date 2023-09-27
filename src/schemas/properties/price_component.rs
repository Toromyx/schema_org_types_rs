use super::*;
/// <https://schema.org/priceComponent>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PriceComponentProperty {
    #[cfg(any(
        any(
            feature = "unit-price-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    UnitPriceSpecification(UnitPriceSpecification),
}

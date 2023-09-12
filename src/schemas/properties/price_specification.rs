use super::*;
/// One or more detailed price specifications, indicating the unit price and delivery or payment charges.
///
/// https://schema.org/priceSpecification
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PriceSpecificationProperty {
    #[cfg(any(
        feature = "price-specification-schema",
        feature = "general-schema-section"
    ))]
    PriceSpecification(PriceSpecification),
}

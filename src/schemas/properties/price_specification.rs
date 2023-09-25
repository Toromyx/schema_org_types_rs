use super::*;
/// One or more detailed price specifications, indicating the unit price and delivery or payment charges.
///
/// https://schema.org/priceSpecification
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PriceSpecificationProperty {
    #[cfg(any(
        any(
            feature = "price-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    PriceSpecification(PriceSpecification),
}

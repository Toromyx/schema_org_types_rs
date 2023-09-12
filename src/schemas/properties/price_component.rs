use super::*;
/// This property links to all [[UnitPriceSpecification]] nodes that apply in parallel for the [[CompoundPriceSpecification]] node.
///
/// https://schema.org/priceComponent
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PriceComponentProperty {
    #[cfg(any(
        feature = "unit-price-specification-schema",
        feature = "general-schema-section"
    ))]
    UnitPriceSpecification(UnitPriceSpecification),
}

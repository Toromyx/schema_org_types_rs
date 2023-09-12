use super::*;
/// Defines the type of a price specified for an offered product, for example a list price, a (temporary) sale price or a manufacturer suggested retail price. If multiple prices are specified for an offer the [[priceType]] property can be used to identify the type of each such specified price. The value of priceType can be specified as a value from enumeration PriceTypeEnumeration or as a free form text string for price types that are not already predefined in PriceTypeEnumeration.
///
/// https://schema.org/priceType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PriceTypeProperty {
    #[cfg(any(
        feature = "price-type-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    PriceTypeEnumeration(PriceTypeEnumeration),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}

use super::*;
/// <https://schema.org/priceType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PriceTypeProperty {
    #[cfg(any(
        any(
            feature = "price-type-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    PriceTypeEnumeration(PriceTypeEnumeration),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}

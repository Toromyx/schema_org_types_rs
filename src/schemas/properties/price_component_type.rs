use super::*;
/// <https://schema.org/priceComponentType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PriceComponentTypeProperty {
    #[cfg(any(
        any(
            feature = "price-component-type-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    PriceComponentTypeEnumeration(PriceComponentTypeEnumeration),
}

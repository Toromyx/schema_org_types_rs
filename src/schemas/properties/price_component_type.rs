use super::*;
/// Identifies a price component (for example, a line item on an invoice), part of the total price for an offer.
///
/// https://schema.org/priceComponentType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PriceComponentTypeProperty {
    #[cfg(any(
        feature = "price-component-type-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    PriceComponentTypeEnumeration(PriceComponentTypeEnumeration),
}

use super::*;
/// The date after which the price is no longer available.
///
/// https://schema.org/priceValidUntil
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PriceValidUntilProperty {
    #[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
    Date(Date),
}
